use differential_equations::{
    methods::ExplicitRungeKutta, prelude::DiagonallyImplicitRungeKutta, solution::Solution,
};
use rma_kinetics::{
    models::{chemogenetic, constitutive, oscillation, tetoff},
    Solve,
};
use rma_kinetics_common::{get_summary, ModelType, SummaryData};

#[tauri::command(rename_all = "snake_case")]
fn simulate_constitutive_model(
    model: constitutive::Model,
    init_state: constitutive::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<(Solution<f64, constitutive::State<f64>>, Vec<SummaryData>), String> {
    let mut solver = ExplicitRungeKutta::dopri5();

    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| e.to_string())?;

    let summary = get_summary(&solution, ModelType::Constitutive)?;

    Ok((solution, summary))
}

#[tauri::command(rename_all = "snake_case")]
fn simulate_oscillating_model(
    model: oscillation::Model,
    init_state: oscillation::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<(Solution<f64, oscillation::State<f64>>, Vec<SummaryData>), String> {
    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| e.to_string())?;
    let summary = get_summary(&solution, ModelType::Oscillating)?;
    Ok((solution, summary))
}

#[tauri::command(rename_all = "snake_case")]
fn simulate_tetoff_model(
    model: tetoff::Model,
    init_state: tetoff::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<(Solution<f64, tetoff::State<f64>>, Vec<SummaryData>), String> {
    let mut solver = ExplicitRungeKutta::dopri5();
    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| e.to_string())?;
    let summary = get_summary(&solution, ModelType::TetOff)?;
    Ok((solution, summary))
}

#[tauri::command(rename_all = "snake_case")]
fn simulate_chemogenetic_model(
    model: chemogenetic::Model,
    init_state: chemogenetic::State<f64>,
    t0: f64,
    tf: f64,
    dt: f64,
) -> Result<(Solution<f64, chemogenetic::State<f64>>, Vec<SummaryData>), String> {
    let mut solver = DiagonallyImplicitRungeKutta::kvaerno423().max_rejects(100);
    let solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| e.to_string())?;
    let summary = get_summary(&solution, ModelType::Chemogenetic)?;
    Ok((solution, summary))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            simulate_constitutive_model,
            simulate_tetoff_model,
            simulate_chemogenetic_model,
            simulate_oscillating_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
