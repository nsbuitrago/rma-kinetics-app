use differential_equations::{
    methods::ExplicitRungeKutta, prelude::DiagonallyImplicitRungeKutta, solution::Solution,
    traits::State as StateTrait,
};
use rma_kinetics::{
    models::{chemogenetic, constitutive, oscillation, tetoff, dox, cno},
    SolutionAccess, Solve,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Set up better panic messages for debugging
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    Constitutive,
    TetOff,
    Chemogenetic,
    Oscillating,
}

#[derive(Serialize, Deserialize)]
pub enum SpeciesType {
    #[serde(rename(serialize = "Brain RMA"))]
    BrainRMA,
    #[serde(rename(serialize = "Plasma RMA"))]
    PlasmaRMA,
    #[serde(rename(serialize = "tTA"))]
    Tta,
    #[serde(rename(serialize = "Brain Dox"))]
    BrainDox,
    #[serde(rename(serialize = "Plasma Dox"))]
    PlasmaDox,
    #[serde(rename(serialize = "hM3Dq"))]
    Dreadd,
    #[serde(rename(serialize = "Peritoneal CNO"))]
    PeritonealCno,
    #[serde(rename(serialize = "Brain CNO"))]
    BrainCno,
    #[serde(rename(serialize = "Plasma CNO"))]
    PlasmaCno,
    #[serde(rename(serialize = "Brain CLZ"))]
    BrainClz,
    #[serde(rename(serialize = "Plasma CLZ"))]
    PlasmaClz,
}

#[derive(Serialize, Deserialize)]
pub struct SummaryData {
    species: SpeciesType,
    max_concentration: f64,
    tmax: f64,
}

// ============================================================================
// WASM-specific wrapper types for JS interop
// ============================================================================
// These types match the JavaScript structure exactly and are converted to
// the library types that use Rust-specific types like RangeInclusive.

/// WASM-specific AccessPeriod that uses explicit start/end fields
/// instead of RangeInclusive for proper JS deserialization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmAccessPeriod {
    pub dose: f64,
    pub time: WasmTimeRange,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmTimeRange {
    pub start: f64,
    pub end: f64,
}

impl From<WasmAccessPeriod> for dox::AccessPeriod {
    fn from(wasm: WasmAccessPeriod) -> Self {
        dox::AccessPeriod::new(wasm.dose, wasm.time.start..=wasm.time.end)
    }
}

/// WASM-specific DoxModel wrapper
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmDoxModel {
    pub vehicle_intake: f64,
    pub bioavailability: f64,
    pub absorption: f64,
    pub elimination: f64,
    pub brain_transport: f64,
    pub plasma_transport: f64,
    pub plasma_vd: f64,
    pub schedule: Vec<WasmAccessPeriod>,
    pub dose_concentration: Vec<f64>,
}

impl From<WasmDoxModel> for dox::Model {
    fn from(wasm: WasmDoxModel) -> Self {
        let schedule: Vec<dox::AccessPeriod> = wasm
            .schedule
            .into_iter()
            .map(|period| period.into())
            .collect();
        
        dox::Model {
            vehicle_intake: wasm.vehicle_intake,
            bioavailability: wasm.bioavailability,
            absorption: wasm.absorption,
            elimination: wasm.elimination,
            brain_transport: wasm.brain_transport,
            plasma_transport: wasm.plasma_transport,
            plasma_vd: wasm.plasma_vd,
            schedule,
            dose_concentration: wasm.dose_concentration,
        }
    }
}

/// WASM-specific CnoDose wrapper
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmCnoDose {
    pub mg: f64,
    pub nmol: f64,
    pub time: f64,
}

impl From<WasmCnoDose> for cno::Dose {
    fn from(wasm: WasmCnoDose) -> Self {
        cno::Dose::new(wasm.mg, wasm.time)
    }
}

/// WASM-specific CnoModel wrapper
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmCnoModel {
    pub doses: Vec<WasmCnoDose>,
    pub cno_absorption: f64,
    pub cno_elimination: f64,
    pub cno_reverse_metabolism: f64,
    pub clz_metabolism: f64,
    pub clz_elimination: f64,
    pub cno_brain_transport: f64,
    pub cno_plasma_transport: f64,
    pub clz_brain_transport: f64,
    pub clz_plasma_transport: f64,
    pub cno_plasma_vd: f64,
    pub cno_brain_vd: f64,
    pub clz_plasma_vd: f64,
    pub clz_brain_vd: f64,
}

impl From<WasmCnoModel> for cno::Model {
    fn from(wasm: WasmCnoModel) -> Self {
        let doses: Vec<cno::Dose> = wasm
            .doses
            .into_iter()
            .map(|dose| dose.into())
            .collect();
        
        // Use builder pattern if available, otherwise construct directly
        cno::Model::builder()
            .doses(doses)
            .cno_absorption(wasm.cno_absorption)
            .cno_elimination(wasm.cno_elimination)
            .cno_reverse_metabolism(wasm.cno_reverse_metabolism)
            .clz_metabolism(wasm.clz_metabolism)
            .clz_elimination(wasm.clz_elimination)
            .cno_brain_transport(wasm.cno_brain_transport)
            .cno_plasma_transport(wasm.cno_plasma_transport)
            .clz_brain_transport(wasm.clz_brain_transport)
            .clz_plasma_transport(wasm.clz_plasma_transport)
            .cno_plasma_vd(wasm.cno_plasma_vd)
            .cno_brain_vd(wasm.cno_brain_vd)
            .clz_plasma_vd(wasm.clz_plasma_vd)
            .clz_brain_vd(wasm.clz_brain_vd)
            .build()
            .expect("Failed to build CNO model from WASM wrapper")
    }
}

/// WASM-specific TetOff model wrapper
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmTetoffModel {
    pub rma_prod: f64,
    pub leaky_rma_prod: f64,
    pub rma_bbb_transport: f64,
    pub rma_deg: f64,
    pub tta_prod: f64,
    pub tta_deg: f64,
    pub tta_kd: f64,
    pub tta_cooperativity: f64,
    pub dox_pk_model: WasmDoxModel,
    pub dox_tta_kd: f64,
}

impl From<WasmTetoffModel> for tetoff::Model {
    fn from(wasm: WasmTetoffModel) -> Self {
        let dox_pk_model: dox::Model = wasm.dox_pk_model.into();
        
        tetoff::Model {
            rma_prod: wasm.rma_prod,
            leaky_rma_prod: wasm.leaky_rma_prod,
            rma_bbb_transport: wasm.rma_bbb_transport,
            rma_deg: wasm.rma_deg,
            tta_prod: wasm.tta_prod,
            tta_deg: wasm.tta_deg,
            tta_kd: wasm.tta_kd,
            tta_cooperativity: wasm.tta_cooperativity,
            dox_pk_model,
            dox_tta_kd: wasm.dox_tta_kd,
        }
    }
}

/// WASM-specific Chemogenetic model wrapper
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmChemogeneticModel {
    pub rma_prod: f64,
    pub leaky_rma_prod: f64,
    pub rma_bbb_transport: f64,
    pub rma_deg: f64,
    pub tta_prod: f64,
    pub leaky_tta_prod: f64,
    pub tta_deg: f64,
    pub tta_kd: f64,
    pub tta_cooperativity: f64,
    pub dox_pk_model: WasmDoxModel,
    pub dox_tta_kd: f64,
    pub cno_pk_model: WasmCnoModel,
    pub cno_ec50: f64,
    pub clz_ec50: f64,
    pub cno_cooperativity: f64,
    pub clz_cooperativity: f64,
    pub dreadd_prod: f64,
    pub dreadd_deg: f64,
    pub dreadd_ec50: f64,
    pub dreadd_cooperativity: f64,
}

impl From<WasmChemogeneticModel> for chemogenetic::Model {
    fn from(wasm: WasmChemogeneticModel) -> Self {
        let dox_pk_model: dox::Model = wasm.dox_pk_model.into();
        let cno_pk_model: cno::Model = wasm.cno_pk_model.into();
        
        chemogenetic::Model {
            rma_prod: wasm.rma_prod,
            leaky_rma_prod: wasm.leaky_rma_prod,
            rma_bbb_transport: wasm.rma_bbb_transport,
            rma_deg: wasm.rma_deg,
            tta_prod: wasm.tta_prod,
            leaky_tta_prod: wasm.leaky_tta_prod,
            tta_deg: wasm.tta_deg,
            tta_kd: wasm.tta_kd,
            tta_cooperativity: wasm.tta_cooperativity,
            dox_pk_model,
            dox_tta_kd: wasm.dox_tta_kd,
            cno_pk_model,
            cno_ec50: wasm.cno_ec50,
            clz_ec50: wasm.clz_ec50,
            cno_cooperativity: wasm.cno_cooperativity,
            clz_cooperativity: wasm.clz_cooperativity,
            dreadd_prod: wasm.dreadd_prod,
            dreadd_deg: wasm.dreadd_deg,
            dreadd_ec50: wasm.dreadd_ec50,
            dreadd_cooperativity: wasm.dreadd_cooperativity,
        }
    }
}

fn get_summary<S: StateTrait<f64>>(
    solution: &Solution<f64, S>,
    model_type: ModelType,
) -> Result<Vec<SummaryData>, String>
where
    Solution<f64, S>: SolutionAccess,
{
    let (plasma_rma_tmax, plasma_rma_max) = solution.max_plasma_rma().map_err(|e| e.to_string())?;
    let plasma_rma_summary = SummaryData {
        species: SpeciesType::PlasmaRMA,
        max_concentration: plasma_rma_max,
        tmax: plasma_rma_tmax,
    };
    let (brain_rma_tmax, brain_rma_max) = solution.max_brain_rma().map_err(|e| e.to_string())?;
    let brain_rma_summary = SummaryData {
        species: SpeciesType::BrainRMA,
        max_concentration: brain_rma_max,
        tmax: brain_rma_tmax,
    };

    let mut summary_data = Vec::new();

    if model_type == ModelType::Constitutive || model_type == ModelType::Oscillating {
        summary_data.extend([plasma_rma_summary, brain_rma_summary]);
        return Ok(summary_data);
    }

    let (plasma_dox_tmax, plasma_dox_max) = solution.max_plasma_dox().map_err(|e| e.to_string())?;
    let plasma_dox_summary = SummaryData {
        species: SpeciesType::PlasmaDox,
        max_concentration: plasma_dox_max,
        tmax: plasma_dox_tmax,
    };

    let (brain_dox_tmax, brain_dox_max) = solution.max_brain_dox().map_err(|e| e.to_string())?;
    let brain_dox_summary = SummaryData {
        species: SpeciesType::BrainDox,
        max_concentration: brain_dox_max,
        tmax: brain_dox_tmax,
    };

    let (tta_tmax, tta_max) = solution.max_tta().map_err(|e| e.to_string())?;
    let tta_summary = SummaryData {
        species: SpeciesType::Tta,
        max_concentration: tta_max,
        tmax: tta_tmax,
    };

    summary_data.extend([plasma_dox_summary, brain_dox_summary, tta_summary]);

    if model_type == ModelType::TetOff {
        return Ok(summary_data);
    }

    let (dreadd_tmax, dreadd_max) = solution.max_dreadd().map_err(|e| e.to_string())?;
    let dreadd_summary = SummaryData {
        species: SpeciesType::Dreadd,
        max_concentration: dreadd_max,
        tmax: dreadd_tmax,
    };

    let (peritoneal_cno_tmax, peritoneal_cno_max) =
        solution.max_peritoneal_cno().map_err(|e| e.to_string())?;
    let peritoneal_cno_summary = SummaryData {
        species: SpeciesType::PeritonealCno,
        max_concentration: peritoneal_cno_max,
        tmax: peritoneal_cno_tmax,
    };

    let (plasma_cno_tmax, plasma_cno_max) = solution.max_plasma_cno().map_err(|e| e.to_string())?;
    let plasma_cno_summary = SummaryData {
        species: SpeciesType::PlasmaCno,
        max_concentration: plasma_cno_max,
        tmax: plasma_cno_tmax,
    };

    let (brain_cno_tmax, brain_cno_max) = solution.max_brain_cno().map_err(|e| e.to_string())?;
    let brain_cno_summary = SummaryData {
        species: SpeciesType::BrainCno,
        max_concentration: brain_cno_max,
        tmax: brain_cno_tmax,
    };

    let (plasma_clz_tmax, plasma_clz_max) = solution.max_plasma_clz().map_err(|e| e.to_string())?;
    let plasma_clz_summary = SummaryData {
        species: SpeciesType::PlasmaClz,
        max_concentration: plasma_clz_max,
        tmax: plasma_clz_tmax,
    };

    let (brain_clz_tmax, brain_clz_max) = solution.max_brain_clz().map_err(|e| e.to_string())?;
    let brain_clz_summary = SummaryData {
        species: SpeciesType::BrainClz,
        max_concentration: brain_clz_max,
        tmax: brain_clz_tmax,
    };

    summary_data.extend(vec![
        dreadd_summary,
        peritoneal_cno_summary,
        plasma_cno_summary,
        brain_cno_summary,
        plasma_clz_summary,
        brain_clz_summary,
    ]);

    Ok(summary_data)
}

#[wasm_bindgen]
pub fn simulate_constitutive_model(
    model: JsValue,
    init_state: JsValue,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<JsValue, JsError> {
    web_sys::console::log_1(&"simulate_constitutive_model called in WASM".into());
    web_sys::console::log_2(&"model:".into(), &model);
    web_sys::console::log_2(&"init_state:".into(), &init_state);
    
    let model: constitutive::Model = serde_wasm_bindgen::from_value(model)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize model: {}", e).into());
            JsError::new(&format!("Failed to deserialize model: {}", e))
        })?;
    let init_state: constitutive::State<f64> = serde_wasm_bindgen::from_value(init_state)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize init_state: {}", e).into());
            JsError::new(&format!("Failed to deserialize init_state: {}", e))
        })?;

    web_sys::console::log_1(&"Deserialization successful, starting solver".into());

    let mut solver = ExplicitRungeKutta::dopri5();

    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Solver error: {}", e).into());
            JsError::new(&e.to_string())
        })?;

    web_sys::console::log_1(&"Solver completed, getting summary".into());

    let summary = get_summary(&solution, ModelType::Constitutive)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Summary error: {}", e).into());
            JsError::new(&e)
        })?;

    web_sys::console::log_1(&"Summary completed, serializing result".into());

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to serialize result: {}", e).into());
            JsError::new(&format!("Failed to serialize result: {}", e))
        })
}

#[wasm_bindgen]
pub fn simulate_oscillating_model(
    model: JsValue,
    init_state: JsValue,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<JsValue, JsError> {
    let model: oscillation::Model = serde_wasm_bindgen::from_value(model)
        .map_err(|e| JsError::new(&format!("Failed to deserialize model: {}", e)))?;
    let init_state: oscillation::State<f64> = serde_wasm_bindgen::from_value(init_state)
        .map_err(|e| JsError::new(&format!("Failed to deserialize init_state: {}", e)))?;

    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let summary = get_summary(&solution, ModelType::Oscillating)
        .map_err(|e| JsError::new(&e))?;

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsError::new(&format!("Failed to serialize result: {}", e)))
}

#[wasm_bindgen]
pub fn simulate_tetoff_model(
    model: JsValue,
    init_state: JsValue,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<JsValue, JsError> {
    web_sys::console::log_1(&"simulate_tetoff_model called in WASM".into());
    web_sys::console::log_2(&"model:".into(), &model);
    
    // Deserialize to WASM wrapper type first
    let wasm_model: WasmTetoffModel = serde_wasm_bindgen::from_value(model)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize model: {}", e).into());
            JsError::new(&format!("Failed to deserialize model: {}", e))
        })?;
    
    web_sys::console::log_1(&"Model deserialized to WASM wrapper, converting to library type".into());
    
    // Convert to library type
    let model: tetoff::Model = wasm_model.into();
    
    let init_state: tetoff::State<f64> = serde_wasm_bindgen::from_value(init_state)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize init_state: {}", e).into());
            JsError::new(&format!("Failed to deserialize init_state: {}", e))
        })?;

    web_sys::console::log_1(&"Deserialization successful, starting solver".into());

    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Solver error: {}", e).into());
            JsError::new(&e.to_string())
        })?;
    
    web_sys::console::log_1(&"Solver completed, getting summary".into());
    
    let summary = get_summary(&solution, ModelType::TetOff)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Summary error: {}", e).into());
            JsError::new(&e)
        })?;

    web_sys::console::log_1(&"Summary completed, serializing result".into());

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to serialize result: {}", e).into());
            JsError::new(&format!("Failed to serialize result: {}", e))
        })
}

#[wasm_bindgen]
pub fn simulate_chemogenetic_model(
    model: JsValue,
    init_state: JsValue,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<JsValue, JsError> {
    web_sys::console::log_1(&"simulate_chemogenetic_model called in WASM".into());
    web_sys::console::log_2(&"model:".into(), &model);
    
    // Deserialize to WASM wrapper type first
    let wasm_model: WasmChemogeneticModel = serde_wasm_bindgen::from_value(model)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize model: {}", e).into());
            JsError::new(&format!("Failed to deserialize model: {}", e))
        })?;
    
    web_sys::console::log_1(&"Model deserialized to WASM wrapper, converting to library type".into());
    
    // Convert to library type
    let model: chemogenetic::Model = wasm_model.into();
    
    let init_state: chemogenetic::State<f64> = serde_wasm_bindgen::from_value(init_state)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize init_state: {}", e).into());
            JsError::new(&format!("Failed to deserialize init_state: {}", e))
        })?;

    web_sys::console::log_1(&"Deserialization successful, starting solver".into());

    let mut solver = DiagonallyImplicitRungeKutta::kvaerno423().max_rejects(100);
    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Solver error: {}", e).into());
            JsError::new(&e.to_string())
        })?;
    
    web_sys::console::log_1(&"Solver completed, getting summary".into());
    
    let summary = get_summary(&solution, ModelType::Chemogenetic)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Summary error: {}", e).into());
            JsError::new(&e)
        })?;

    web_sys::console::log_1(&"Summary completed, serializing result".into());

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to serialize result: {}", e).into());
            JsError::new(&format!("Failed to serialize result: {}", e))
        })
}
