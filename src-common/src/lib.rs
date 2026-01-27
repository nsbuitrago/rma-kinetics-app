use differential_equations::{solution::Solution, traits::State as StateTrait};
use rma_kinetics::SolutionAccess;
use serde::{Deserialize, Serialize};

// ============================================================================
// Structs
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct SummaryData {
    pub species: SpeciesType,
    pub max_concentration: f64,
    pub tmax: f64,
}

// ============================================================================
// Enums
// ============================================================================

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

// ============================================================================
// Functions
// ============================================================================

pub fn get_summary<S: StateTrait<f64>>(
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

    let mut summary_data = vec![plasma_rma_summary, brain_rma_summary];

    if model_type == ModelType::Constitutive || model_type == ModelType::Oscillating {
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
