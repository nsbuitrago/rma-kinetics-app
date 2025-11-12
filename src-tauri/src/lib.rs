mod chemogenetic;
mod cno;
mod constitutive;
mod dox;
mod oscillation;
mod solve;
mod tetoff;

use constitutive::ConstitutiveRMA;
use serde::{Deserialize, Serialize};
use solve::{Solution, SolverType};

use crate::{
    chemogenetic::{ChemogeneticRMA, DqConfig},
    cno::{CnoArgs, CnoPKConfig},
    dox::DoxArgs,
    oscillation::OscillatingRMA,
    tetoff::{RmaConfig, TetoffRMA, TtaConfig},
};

#[derive(Serialize, Deserialize)]
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

#[tauri::command(rename_all = "snake_case")]
fn constitutive_model(
    rma_prod_rate: f64,
    rma_rt_rate: f64,
    rma_deg_rate: f64,
    init: Vec<f64>,
    tf: f64,
) -> Solution {
    let model = ConstitutiveRMA::new(rma_prod_rate, rma_rt_rate, rma_deg_rate);
    model.solve(tf, init, SolverType::Bdf)
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

#[tauri::command(rename_all = "snake_case")]
fn oscillating_model(
    rma_prod_rate: f64,
    rma_rt_rate: f64,
    rma_deg_rate: f64,
    frequency: f64,
    init: Vec<f64>,
    tf: f64,
) -> Solution {
    let model = OscillatingRMA::new(rma_prod_rate, rma_rt_rate, rma_deg_rate, frequency);
    model.solve(tf, init, SolverType::Bdf)
}

#[tauri::command(rename_all = "snake_case")]
fn get_brain_rma(solution: Solution) -> Vec<f64> {
    solution.ys.row(0).into_iter().copied().collect()
}

#[tauri::command(rename_all = "snake_case")]
fn get_plasma_rma(solution: Solution) -> Vec<f64> {
    solution.ys.row(1).into_iter().copied().collect()
}

#[tauri::command(rename_all = "snake_case")]
fn get_tta(solution: Solution) -> Vec<f64> {
    solution.ys.row(2).into_iter().copied().collect()
}

#[tauri::command(rename_all = "snake_case")]
fn get_brain_dox(solution: Solution) -> Vec<f64> {
    solution.ys.row(3).into_iter().copied().collect()
}

const CONSTITUTIVE_SPECIES_SUMMARY_INDICES: &[usize; 2] = &[0, 1];
const TETOFF_SPECIES_SUMMARY_INDICES: &[usize; 4] = &[0, 1, 2, 3];
const CHEMO_SPECIES_SUMMARY_INDICES: &[usize; 6] = &[0, 1, 2, 3, 7, 9];

#[tauri::command(rename_all = "snake_case")]
fn get_summary(solution: Solution) -> Vec<Vec<f64>> {
    match solution.model {
        ModelType::Constitutive => get_summary_data(solution, CONSTITUTIVE_SPECIES_SUMMARY_INDICES),
        ModelType::TetOff => get_summary_data(solution, TETOFF_SPECIES_SUMMARY_INDICES),
        ModelType::Chemogenetic => get_summary_data(solution, CHEMO_SPECIES_SUMMARY_INDICES),
        ModelType::Oscillating => get_summary_data(solution, CONSTITUTIVE_SPECIES_SUMMARY_INDICES),
    }
}

fn get_summary_data(solution: Solution, indices: &[usize]) -> Vec<Vec<f64>> {
    let mut summary = Vec::new();

    indices.iter().for_each(|idx| {
        let row_data = solution.ys.row(*idx);
        let max_concentration = row_data.max();
        let tmax_idx = row_data
            .iter()
            .position(|x| x == &max_concentration)
            .unwrap_or_default();
        let tmax = solution.ts[tmax_idx];

        let summary_data = vec![max_concentration, tmax];
        summary.push(summary_data);
    });

    summary
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            constitutive_model,
            tetoff_model,
            chemogenetic_model,
            oscillating_model,
            get_brain_rma,
            get_plasma_rma,
            get_tta,
            get_brain_dox,
            get_summary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
