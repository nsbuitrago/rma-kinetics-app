use serde::{Deserialize, Serialize};

const CNO_MW: f64 = 342.8; // g/mol

#[derive(Serialize, Deserialize)]
pub struct CnoArgs {
    pub dose: f64,
    pub t0: f64,
    pub cno_absorption_rate: f64,
    pub cno_elimination_rate: f64,
    pub cno_reverse_metabolism_rate: f64,
    pub clz_metabolism_rate: f64,
    pub cno_brain_transport_rate: f64,
    pub cno_plasma_transport_rate: f64,
    pub clz_brain_transport_rate: f64,
    pub clz_plasma_transport_rate: f64,
    pub clz_elimination_rate: f64,
    pub cno_plasma_vd: f64,
    pub cno_brain_vd: f64,
    pub clz_plasma_vd: f64,
    pub clz_brain_vd: f64,
    pub cno_ec50: f64,
    pub clz_ec50: f64,
    pub cno_coop: f64,
    pub clz_coop: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CnoPKConfig {
    pub dose: f64,
    pub t0: f64,
    pub cno_absorption_rate: f64,
    pub cno_elimination_rate: f64,
    pub cno_reverse_metabolism_rate: f64,
    pub clz_metabolism_rate: f64,
    pub cno_brain_transport_rate: f64,
    pub cno_plasma_transport_rate: f64,
    pub clz_brain_transport_rate: f64,
    pub clz_plasma_transport_rate: f64,
    pub clz_elimination_rate: f64,
    pub cno_plasma_vd: f64,
    pub cno_brain_vd: f64,
    pub clz_plasma_vd: f64,
    pub clz_brain_vd: f64,
    pub cno_ec50: f64,
    pub clz_ec50: f64,
    pub cno_coop: f64,
    pub clz_coop: f64,
    pub cno_nmol: f64,
}

impl CnoPKConfig {
    pub fn new(cno_args: CnoArgs) -> Self {
        Self {
            dose: cno_args.dose,
            t0: cno_args.t0,
            cno_absorption_rate: cno_args.cno_absorption_rate,
            cno_elimination_rate: cno_args.cno_elimination_rate,
            cno_reverse_metabolism_rate: cno_args.cno_reverse_metabolism_rate,
            clz_metabolism_rate: cno_args.clz_metabolism_rate,
            cno_brain_transport_rate: cno_args.cno_brain_transport_rate,
            cno_plasma_transport_rate: cno_args.cno_plasma_transport_rate,
            clz_brain_transport_rate: cno_args.clz_brain_transport_rate,
            clz_plasma_transport_rate: cno_args.clz_plasma_transport_rate,
            clz_elimination_rate: cno_args.clz_elimination_rate,
            cno_plasma_vd: cno_args.cno_plasma_vd,
            cno_brain_vd: cno_args.cno_brain_vd,
            clz_plasma_vd: cno_args.clz_plasma_vd,
            clz_brain_vd: cno_args.clz_brain_vd,
            cno_ec50: cno_args.cno_ec50,
            clz_ec50: cno_args.clz_ec50,
            cno_coop: cno_args.cno_coop,
            clz_coop: cno_args.clz_coop,
            cno_nmol: cno_args.dose / CNO_MW * 1e6,
        }
    }
}
