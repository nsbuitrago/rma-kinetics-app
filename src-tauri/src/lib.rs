mod chemogenetic;
mod cno;
mod dox;
// mod oscillation;
mod solve;
mod tetoff;

use serde::{Deserialize, Serialize};
use solve::{Solution, SolverType};

use crate::{
    chemogenetic::{ChemogeneticRMA, DqConfig},
    cno::CnoArgs,
    dox::DoxArgs,
    // oscillation::OscillatingRMA,
    tetoff::{RmaConfig, TetoffRMA, TtaConfig},
};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    Constitutive,
    TetOff,
    Chemogenetic,
    Oscillating,
}

// enum Species {
//     BrainRMA,
//     PlasmaRMA,
// }

// const SPECIES: &[&str; 7] = &[
//     "Brain RMA",
//     "Plasma RMA",
//     "tTA",
//     "Dox",
//     "hM3Dq",
//     "CNO",
//     "CLZ",
// ];
//
use differential_equations::{
    methods::ExplicitRungeKutta, solution::Solution as DESolution, traits::State as StateTrait,
};
use rma_kinetics::{
    models::{constitutive, oscillation, tetoff as rk_tetoff},
    SolutionAccess, Solve,
};

#[tauri::command(rename_all = "snake_case")]
fn simulate_constitutive_model(
    model: constitutive::Model,
    init_state: constitutive::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> (DESolution<f64, constitutive::State<f64>>, Vec<SummaryData>) {
    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model.solve(t0, tf, dt, init_state, &mut solver).unwrap();
    let summary = get_summary(&solution, ModelType::Constitutive);

    (solution, summary)
}

#[tauri::command(rename_all = "snake_case")]
fn simulate_oscillating_model(
    model: oscillation::Model,
    init_state: oscillation::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> (DESolution<f64, oscillation::State<f64>>, Vec<SummaryData>) {
    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model.solve(t0, tf, dt, init_state, &mut solver).unwrap();
    let summary = get_summary(&solution, ModelType::Oscillating);
    (solution, summary)
}

#[tauri::command(rename_all = "snake_case")]
fn simulate_tetoff_model(
    model: rk_tetoff::Model,
    init_state: rk_tetoff::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> DESolution<f64, rk_tetoff::State<f64>> {
    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model.solve(t0, tf, dt, init_state, &mut solver);
    solution.unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn tetoff_model(
    rma_config: RmaConfig,
    tta_config: TtaConfig,
    dox_config: DoxArgs,
    init: Vec<f64>,
    tf: f64,
) -> Solution {
    let model = TetoffRMA::new(rma_config, tta_config, dox_config);
    model.solve(tf, init, SolverType::Bdf)
}

#[tauri::command(rename_all = "snake_case")]
fn chemogenetic_model(
    rma_config: RmaConfig,
    tta_config: TtaConfig,
    dq_config: DqConfig,
    dox_config: DoxArgs,
    cno_config: CnoArgs,
    init: Vec<f64>,
    tf: f64,
) -> Solution {
    let model = ChemogeneticRMA::new(rma_config, tta_config, dq_config, dox_config, cno_config);
    model.solve(tf, init, SolverType::Bdf)
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

fn get_summary<S: StateTrait<f64>>(
    solution: &DESolution<f64, S>,
    model_type: ModelType,
) -> Vec<SummaryData>
where
    DESolution<f64, S>: SolutionAccess,
{
    let (plasma_rma_tmax, plasma_rma_max) = solution.max_plasma_rma().unwrap();
    let plasma_rma_summary = SummaryData {
        species: SpeciesType::PlasmaRMA,
        max_concentration: plasma_rma_max,
        tmax: plasma_rma_tmax,
    };
    let (brain_rma_tmax, brain_rma_max) = solution.max_brain_rma().unwrap();
    let brain_rma_summary = SummaryData {
        species: SpeciesType::BrainRMA,
        max_concentration: brain_rma_max,
        tmax: brain_rma_tmax,
    };

    let mut summary_data = Vec::new();

    if model_type == ModelType::Constitutive || model_type == ModelType::Oscillating {
        summary_data.extend([plasma_rma_summary, brain_rma_summary]);
        return summary_data;
    }

    let (plasma_dox_tmax, plasma_dox_max) = solution.max_plasma_dox().unwrap();
    let plasma_dox_summary = SummaryData {
        species: SpeciesType::PlasmaDox,
        max_concentration: plasma_dox_max,
        tmax: plasma_dox_tmax,
    };

    let (brain_dox_tmax, brain_dox_max) = solution.max_brain_dox().unwrap();
    let brain_dox_summary = SummaryData {
        species: SpeciesType::BrainDox,
        max_concentration: brain_dox_max,
        tmax: brain_dox_tmax,
    };

    let (tta_tmax, tta_max) = solution.max_tta().unwrap();
    let tta_summary = SummaryData {
        species: SpeciesType::Tta,
        max_concentration: tta_max,
        tmax: tta_tmax,
    };

    summary_data.extend([plasma_dox_summary, brain_dox_summary, tta_summary]);

    if model_type == ModelType::TetOff {
        return summary_data;
    }

    let (dreadd_tmax, dreadd_max) = solution.max_dreadd().unwrap();
    let dreadd_summary = SummaryData {
        species: SpeciesType::Dreadd,
        max_concentration: dreadd_max,
        tmax: dreadd_tmax,
    };

    let (peritoneal_cno_tmax, peritoneal_cno_max) = solution.max_peritoneal_cno().unwrap();
    let peritoneal_cno_summary = SummaryData {
        species: SpeciesType::PeritonealCno,
        max_concentration: peritoneal_cno_max,
        tmax: peritoneal_cno_tmax,
    };

    let (plasma_cno_tmax, plasma_cno_max) = solution.max_plasma_cno().unwrap();
    let plasma_cno_summary = SummaryData {
        species: SpeciesType::PlasmaCno,
        max_concentration: plasma_cno_max,
        tmax: plasma_cno_tmax,
    };

    let (brain_cno_tmax, brain_cno_max) = solution.max_brain_cno().unwrap();
    let brain_cno_summary = SummaryData {
        species: SpeciesType::BrainCno,
        max_concentration: brain_cno_max,
        tmax: brain_cno_tmax,
    };

    let (plasma_clz_tmax, plasma_clz_max) = solution.max_plasma_clz().unwrap();
    let plasma_clz_summary = SummaryData {
        species: SpeciesType::PlasmaClz,
        max_concentration: plasma_clz_max,
        tmax: plasma_clz_tmax,
    };

    let (brain_clz_tmax, brain_clz_max) = solution.max_brain_clz().unwrap();
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

    summary_data
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            simulate_constitutive_model,
            simulate_tetoff_model,
            tetoff_model,
            chemogenetic_model,
            simulate_oscillating_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
