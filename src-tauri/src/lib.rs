use differential_equations::{
    methods::ExplicitRungeKutta, prelude::DiagonallyImplicitRungeKutta, solution::Solution,
};
use rma_kinetics::{
    models::{chemogenetic, constitutive, oscillation, tetoff},
    ApplyNoise, Solve, ToDataFrame,
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
    noise_level: f64,
) -> Result<(Solution<f64, oscillation::State<f64>>, Vec<SummaryData>), String> {
    let mut solver = ExplicitRungeKutta::dopri5();
    let mut solution = model
        .solve(t0, tf, dt, init_state, &mut solver)
        .map_err(|e| e.to_string())?;

    if noise_level > 0.0 {
        solution.apply_noise(noise_level);
    }

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

#[tauri::command(rename_all = "snake_case")]
async fn export_csv(
    app: tauri::AppHandle,
    solution: serde_json::Value,
    model_type: ModelType,
) -> Result<(), String> {
    use polars::prelude::SerWriter;
    use tauri_plugin_dialog::DialogExt;

    // Deserialize solution and convert to DataFrame based on model type
    let mut df = match model_type {
        ModelType::Constitutive => {
            let sol: Solution<f64, constitutive::State<f64>> =
                serde_json::from_value(solution).map_err(|e| e.to_string())?;
            sol.to_dataframe()
        }
        ModelType::Oscillating => {
            let sol: Solution<f64, oscillation::State<f64>> =
                serde_json::from_value(solution).map_err(|e| e.to_string())?;
            sol.to_dataframe()
        }
        ModelType::TetOff => {
            let sol: Solution<f64, tetoff::State<f64>> =
                serde_json::from_value(solution).map_err(|e| e.to_string())?;
            sol.to_dataframe()
        }
        ModelType::Chemogenetic => {
            let sol: Solution<f64, chemogenetic::State<f64>> =
                serde_json::from_value(solution).map_err(|e| e.to_string())?;
            sol.to_dataframe()
        }
    }
    .map_err(|e| e.to_string())?;

    // Convert DataFrame to CSV bytes
    let mut buf = Vec::new();
    polars::io::csv::write::CsvWriter::new(&mut buf)
        .finish(&mut df)
        .map_err(|e| e.to_string())?;

    // Show native save dialog
    let file_path = app
        .dialog()
        .file()
        .add_filter("CSV", &["csv"])
        .set_file_name("solution.csv")
        .blocking_save_file()
        .ok_or("Save dialog cancelled")?;

    // Write to file
    let path = file_path.as_path().ok_or("Invalid file path")?;
    std::fs::write(path, &buf).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
            export_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
