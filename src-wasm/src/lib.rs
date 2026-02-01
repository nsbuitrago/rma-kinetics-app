use differential_equations::{
    methods::ExplicitRungeKutta,
    prelude::{DiagonallyImplicitRungeKutta, Solution},
};
use rma_kinetics::{
    models::{chemogenetic, cno, constitutive, dox, oscillation, tetoff},
    ApplyNoise, Solve,
};
use rma_kinetics_common::{get_summary, ModelType};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Set up better panic messages for debugging
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
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
        let doses: Vec<cno::Dose> = wasm.doses.into_iter().map(|dose| dose.into()).collect();

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

    let model: constitutive::Model = serde_wasm_bindgen::from_value(model).map_err(|e| {
        web_sys::console::error_1(&format!("Failed to deserialize model: {}", e).into());
        JsError::new(&format!("Failed to deserialize model: {}", e))
    })?;
    let init_state: constitutive::State<f64> =
        serde_wasm_bindgen::from_value(init_state).map_err(|e| {
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

    let summary = get_summary(&solution, ModelType::Constitutive).map_err(|e| {
        web_sys::console::error_1(&format!("Summary error: {}", e).into());
        JsError::new(&e)
    })?;

    web_sys::console::log_1(&"Summary completed, serializing result".into());

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result).map_err(|e| {
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
    noise_level: f64,
) -> Result<JsValue, JsError> {
    let model: oscillation::Model = serde_wasm_bindgen::from_value(model)
        .map_err(|e| JsError::new(&format!("Failed to deserialize model: {}", e)))?;
    let init_state: oscillation::State<f64> = serde_wasm_bindgen::from_value(init_state)
        .map_err(|e| JsError::new(&format!("Failed to deserialize init_state: {}", e)))?;

    let mut solver = ExplicitRungeKutta::dopri5();
    let mut solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| JsError::new(&e.to_string()))?;

    if noise_level > 0.0 {
        solution.apply_noise(noise_level);
    }

    let summary = get_summary(&solution, ModelType::Oscillating).map_err(|e| JsError::new(&e))?;

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
    let wasm_model: WasmTetoffModel = serde_wasm_bindgen::from_value(model).map_err(|e| {
        web_sys::console::error_1(&format!("Failed to deserialize model: {}", e).into());
        JsError::new(&format!("Failed to deserialize model: {}", e))
    })?;

    web_sys::console::log_1(
        &"Model deserialized to WASM wrapper, converting to library type".into(),
    );

    // Convert to library type
    let model: tetoff::Model = wasm_model.into();

    let init_state: tetoff::State<f64> =
        serde_wasm_bindgen::from_value(init_state).map_err(|e| {
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

    let summary = get_summary(&solution, ModelType::TetOff).map_err(|e| {
        web_sys::console::error_1(&format!("Summary error: {}", e).into());
        JsError::new(&e)
    })?;

    web_sys::console::log_1(&"Summary completed, serializing result".into());

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result).map_err(|e| {
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
    let wasm_model: WasmChemogeneticModel = serde_wasm_bindgen::from_value(model).map_err(|e| {
        web_sys::console::error_1(&format!("Failed to deserialize model: {}", e).into());
        JsError::new(&format!("Failed to deserialize model: {}", e))
    })?;

    web_sys::console::log_1(
        &"Model deserialized to WASM wrapper, converting to library type".into(),
    );

    // Convert to library type
    let model: chemogenetic::Model = wasm_model.into();

    let init_state: chemogenetic::State<f64> =
        serde_wasm_bindgen::from_value(init_state).map_err(|e| {
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

    let summary = get_summary(&solution, ModelType::Chemogenetic).map_err(|e| {
        web_sys::console::error_1(&format!("Summary error: {}", e).into());
        JsError::new(&e)
    })?;

    web_sys::console::log_1(&"Summary completed, serializing result".into());

    let result = (solution, summary);
    serde_wasm_bindgen::to_value(&result).map_err(|e| {
        web_sys::console::error_1(&format!("Failed to serialize result: {}", e).into());
        JsError::new(&format!("Failed to serialize result: {}", e))
    })
}

/// Export a solution to CSV and trigger browser download.
///
/// # Arguments
/// * `solution` - The solution object from a simulation
/// * `model_type` - The model type (Constitutive, Oscillating, TetOff, or Chemogenetic)
/// * `filename` - Optional filename for the download (defaults to "solution.csv")
#[wasm_bindgen]
pub fn export_csv(
    solution: JsValue,
    model_type: JsValue,
    filename: Option<String>,
) -> Result<(), JsError> {
    use polars::prelude::SerWriter;
    use rma_kinetics::ToDataFrame;

    web_sys::console::log_1(&"export_csv called".into());

    // Deserialize model type
    let model_type: rma_kinetics_common::ModelType = serde_wasm_bindgen::from_value(model_type)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to deserialize model_type: {}", e).into());
            JsError::new(&format!("Failed to deserialize model_type: {}", e))
        })?;

    web_sys::console::log_1(&format!("Model type: {:?}", model_type).into());

    // Deserialize solution and convert to DataFrame based on model type
    let mut df = match model_type {
        rma_kinetics_common::ModelType::Constitutive => {
            let sol: Solution<f64, constitutive::State<f64>> =
                serde_wasm_bindgen::from_value(solution).map_err(|e| {
                    web_sys::console::error_1(
                        &format!("Failed to deserialize solution: {}", e).into(),
                    );
                    JsError::new(&format!("Failed to deserialize solution: {}", e))
                })?;
            sol.to_dataframe()
        }
        rma_kinetics_common::ModelType::Oscillating => {
            let sol: Solution<f64, oscillation::State<f64>> =
                serde_wasm_bindgen::from_value(solution).map_err(|e| {
                    web_sys::console::error_1(
                        &format!("Failed to deserialize solution: {}", e).into(),
                    );
                    JsError::new(&format!("Failed to deserialize solution: {}", e))
                })?;
            sol.to_dataframe()
        }
        rma_kinetics_common::ModelType::TetOff => {
            let sol: Solution<f64, tetoff::State<f64>> = serde_wasm_bindgen::from_value(solution)
                .map_err(|e| {
                web_sys::console::error_1(&format!("Failed to deserialize solution: {}", e).into());
                JsError::new(&format!("Failed to deserialize solution: {}", e))
            })?;
            sol.to_dataframe()
        }
        rma_kinetics_common::ModelType::Chemogenetic => {
            let sol: Solution<f64, chemogenetic::State<f64>> =
                serde_wasm_bindgen::from_value(solution).map_err(|e| {
                    web_sys::console::error_1(
                        &format!("Failed to deserialize solution: {}", e).into(),
                    );
                    JsError::new(&format!("Failed to deserialize solution: {}", e))
                })?;
            sol.to_dataframe()
        }
    }
    .map_err(|e| {
        web_sys::console::error_1(&format!("Failed to convert to dataframe: {}", e).into());
        JsError::new(&format!("Failed to convert to dataframe: {}", e))
    })?;

    web_sys::console::log_1(&"DataFrame created successfully".into());

    // Convert DataFrame to CSV string
    let mut buf = Vec::new();
    polars::io::csv::write::CsvWriter::new(&mut buf)
        .finish(&mut df)
        .map_err(|e| {
            web_sys::console::error_1(&format!("Failed to write CSV: {}", e).into());
            JsError::new(&format!("Failed to write CSV: {}", e))
        })?;

    let csv_string = String::from_utf8(buf).map_err(|e| {
        web_sys::console::error_1(&format!("Invalid UTF-8 in CSV: {}", e).into());
        JsError::new(&format!("Invalid UTF-8 in CSV: {}", e))
    })?;

    web_sys::console::log_1(&format!("CSV generated, {} bytes", csv_string.len()).into());

    // Trigger browser download
    trigger_download(
        &csv_string,
        &filename.unwrap_or_else(|| "solution.csv".to_string()),
    )?;

    web_sys::console::log_1(&"Download triggered successfully".into());

    Ok(())
}

/// Helper function to trigger a browser download of CSV data.
fn trigger_download(content: &str, filename: &str) -> Result<(), JsError> {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let window = web_sys::window().ok_or_else(|| JsError::new("No window object"))?;
    let document = window
        .document()
        .ok_or_else(|| JsError::new("No document object"))?;

    // Create Blob from CSV content
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(content));

    let opts = BlobPropertyBag::new();
    opts.set_type("text/csv;charset=utf-8");

    let blob = Blob::new_with_str_sequence_and_options(&parts, &opts)
        .map_err(|_| JsError::new("Failed to create Blob"))?;

    // Create object URL
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|_| JsError::new("Failed to create object URL"))?;

    // Create and click anchor element
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|_| JsError::new("Failed to create anchor element"))?
        .dyn_into()
        .map_err(|_| JsError::new("Failed to cast to anchor"))?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    // Cleanup
    Url::revoke_object_url(&url).map_err(|_| JsError::new("Failed to revoke object URL"))?;

    Ok(())
}
