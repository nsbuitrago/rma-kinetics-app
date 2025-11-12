use serde::{Deserialize, Serialize};

const DOX_MW: f64 = 444.4; // g/mol

#[derive(Serialize, Deserialize)]
pub struct DoxArgs {
    pub dose: f64,
    pub t0: f64,
    pub t1: f64,
    pub vehicle_intake_rate: f64,
    pub bioavailability: f64,
    pub absorption_rate: f64,
    pub elimination_rate: f64,
    pub brain_transport_rate: f64,
    pub plasma_transport_rate: f64,
    pub plasma_vd: f64,
    pub dox_kd: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct DoxPKConfig {
    dose: f64,
    pub t0: f64,
    pub t1: f64,
    vehicle_intake_rate: f64,
    bioavailability: f64,
    pub absorption_rate: f64,
    pub elimination_rate: f64,
    pub brain_transport_rate: f64,
    pub plasma_transport_rate: f64,
    plasma_vd: f64,
    pub intake_rate: f64,
    plasma_dox_ss: f64,
    brain_dox_ss: f64,
    pub dox_kd: Option<f64>,
}

impl DoxPKConfig {
    pub fn new(dox_args: DoxArgs) -> Self {
        let intake_rate = dox_args.vehicle_intake_rate * dox_args.bioavailability * dox_args.dose
            / (DOX_MW * dox_args.plasma_vd)
            * 1e6;
        let plasma_dox_ss = dox_args.absorption_rate * intake_rate / dox_args.elimination_rate;
        let brain_dox_ss =
            dox_args.brain_transport_rate * plasma_dox_ss / dox_args.plasma_transport_rate;

        Self {
            dose: dox_args.dose,
            t0: dox_args.t0,
            t1: dox_args.t1,
            vehicle_intake_rate: dox_args.vehicle_intake_rate,
            bioavailability: dox_args.bioavailability,
            absorption_rate: dox_args.absorption_rate,
            elimination_rate: dox_args.elimination_rate,
            brain_transport_rate: dox_args.brain_transport_rate,
            plasma_transport_rate: dox_args.plasma_transport_rate,
            plasma_vd: dox_args.plasma_vd,
            intake_rate,
            plasma_dox_ss,
            brain_dox_ss,
            dox_kd: dox_args.dox_kd,
        }
    }
}
