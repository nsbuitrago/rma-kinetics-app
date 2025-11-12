use crate::{
    cno::{CnoArgs, CnoPKConfig},
    dox::DoxPKConfig,
    tetoff::{RmaConfig, TtaConfig},
    DoxArgs, ModelType, Solution, SolverType,
};
use diffsol::{op::bdf, MatrixCommon, NalgebraLU, NalgebraMat, OdeBuilder, OdeSolverMethod};
use serde::{Deserialize, Serialize};

type LinearSolver = NalgebraLU<f64>;

#[derive(Serialize, Deserialize)]
pub struct DqConfig {
    prod_rate: f64,
    deg_rate: f64,
    ec50: f64,
    coop: f64,
}

pub struct ChemogeneticRMA {
    rma_config: RmaConfig,
    tta_config: TtaConfig,
    dq_config: DqConfig,
    dox_config: DoxPKConfig,
    cno_config: CnoPKConfig,
    n_states: usize,
}

impl ChemogeneticRMA {
    /// Create new model of Tet-gated RMA expression.
    pub fn new(
        rma_config: RmaConfig,
        tta_config: TtaConfig,
        dq_config: DqConfig,
        dox_config: DoxArgs,
        cno_config: CnoArgs,
    ) -> Self {
        let full_dox_config = DoxPKConfig::new(dox_config);
        let full_cno_config = CnoPKConfig::new(cno_config);
        Self {
            rma_config,
            tta_config,
            dq_config,
            dox_config: full_dox_config,
            cno_config: full_cno_config,
            n_states: 11,
        }
    }

    pub fn solve(&self, tf: f64, init: Vec<f64>, solver_type: SolverType) -> Solution {
        let problem = OdeBuilder::<NalgebraMat<f64>>::new()
            // .p(vec![self.prod_rate, self.rt_rate, self.deg_rate])
            .rhs_implicit(
                |species, _params, _t, diff| {
                    // species vector is in the following order
                    // 0. brain RMA
                    // 1. plasma RMA
                    // 2. tTA
                    // 3. brain dox
                    // 4. plasma dox
                    // 5. hM3Dq
                    // 6. peritoneal CNO
                    // 7. brain CNO
                    // 8. plasma CNO
                    // 9. brain CLZ
                    // 10. plasma CLZ

                    let brain_dox_efflux = self.dox_config.plasma_transport_rate * species[3];
                    let plasma_dox_efflux = self.dox_config.brain_transport_rate * species[4];

                    let dox_intake = if _t >= self.dox_config.t0 && _t < self.dox_config.t1 {
                        self.dox_config.intake_rate
                    } else {
                        0.
                    };

                    let active_tta_frac =
                        1. / (1. + species[3] / self.dox_config.dox_kd.unwrap_or_default());

                    let ta_modulator = (active_tta_frac * species[2] / self.tta_config.tta_kd)
                        .powi(self.tta_config.tta_coop);

                    let brain_rma_efflux = self.rma_config.rt_rate * species[0];

                    // brain RMA
                    diff[0] = ((self.rma_config.leaky_prod_rate
                        + (self.rma_config.prod_rate * ta_modulator))
                        / (1. + ta_modulator))
                        - brain_rma_efflux;

                    // plasma RMA
                    diff[1] = brain_rma_efflux - (self.rma_config.deg_rate * species[1]);

                    // neuronal activity induced tTA expression
                    let cno_ec50_hill =
                        (species[7] / self.cno_config.cno_brain_vd / self.cno_config.cno_ec50)
                            .powf(self.cno_config.cno_coop);
                    let clz_ec50_hill =
                        (species[9] / self.cno_config.clz_brain_vd / self.cno_config.clz_ec50)
                            .powf(self.cno_config.clz_coop);
                    let active_dread_frac =
                        (cno_ec50_hill + clz_ec50_hill) / (1. + cno_ec50_hill + clz_ec50_hill);
                    let dreadd_modulator = (active_dread_frac * species[5] / self.dq_config.ec50)
                        .powf(self.dq_config.coop);

                    // neuronal activity induced tTA
                    diff[2] = ((self.tta_config.leaky_prod_rate
                        + (self.tta_config.prod_rate * dreadd_modulator))
                        / (1. + dreadd_modulator))
                        - (self.tta_config.deg_rate * species[2]);

                    // plasma and brain dox
                    diff[3] = plasma_dox_efflux - brain_dox_efflux;
                    diff[4] = (self.dox_config.absorption_rate * dox_intake)
                        - (self.dox_config.elimination_rate * species[4])
                        - plasma_dox_efflux
                        + brain_dox_efflux;

                    // constitutively expressed DREADD
                    diff[5] = self.dq_config.prod_rate - (self.dq_config.deg_rate * species[5]);

                    // CNO kinetics
                    // peritoneal CNO efflux
                    let peritoneal_cno_efflux = self.cno_config.cno_absorption_rate * species[6];

                    // brain CNO fluxes
                    let brain_cno_influx = self.cno_config.cno_brain_transport_rate * species[8];
                    let brain_cno_efflux = self.cno_config.cno_plasma_transport_rate * species[7];

                    // plasma CLZ fluxes
                    let plasma_clz_influx =
                        self.cno_config.cno_reverse_metabolism_rate * species[8];
                    let plasma_clz_efflux = self.cno_config.clz_metabolism_rate * species[10];

                    // brain CLZ fluxes
                    let brain_clz_influx = self.cno_config.clz_brain_transport_rate * species[10];
                    let brain_clz_efflux = self.cno_config.clz_plasma_transport_rate * species[9];

                    diff[6] = -peritoneal_cno_efflux; // peritoneal CNO
                    diff[7] = brain_cno_influx - brain_cno_efflux; // brain CNO

                    // plasma CNO
                    diff[8] = peritoneal_cno_efflux
                        - (self.cno_config.cno_elimination_rate * species[8])
                        - brain_cno_influx
                        + brain_cno_efflux
                        - plasma_clz_influx
                        + plasma_clz_efflux;

                    diff[9] = brain_clz_influx - brain_clz_efflux; // brain CLZ

                    // plasma CLZ
                    diff[10] = plasma_clz_influx
                        - plasma_clz_efflux
                        - (self.cno_config.clz_elimination_rate * species[10])
                        - brain_clz_influx
                        + brain_clz_efflux;
                },
                |species, _params, _t, jac_vec, diff| {
                    // Dox/RMA Module terms
                    let n_dox = self.tta_config.tta_coop;
                    let n_dox_f = n_dox as f64;
                    let kd_dox = self.dox_config.dox_kd.unwrap_or_default();

                    // active_tta_frac = kd_dox / (kd_dox + species[3])
                    let active_tta_frac_term = 1. / (1. + species[3] / kd_dox);

                    let base_val_dox = active_tta_frac_term * species[2] / self.tta_config.tta_kd;
                    let ta_modulator = base_val_dox.powi(n_dox);

                    // CNO/CLZ/tTA Module terms
                    let n_cno = self.cno_config.cno_coop;
                    let n_clz = self.cno_config.clz_coop;
                    let n_dq = self.dq_config.coop;

                    let cno_vd = self.cno_config.cno_brain_vd;
                    let clz_vd = self.cno_config.clz_brain_vd;

                    let cno_hill_base = species[7] / cno_vd / self.cno_config.cno_ec50;
                    let clz_hill_base = species[9] / clz_vd / self.cno_config.clz_ec50;

                    let cno_ec50_hill = cno_hill_base.powf(n_cno);
                    let clz_ec50_hill = clz_hill_base.powf(n_clz);

                    let denominator_dread = 1. + cno_ec50_hill + clz_ec50_hill;
                    let active_dread_frac = (cno_ec50_hill + clz_ec50_hill) / denominator_dread;

                    let base_val_dq = active_dread_frac * species[5] / self.dq_config.ec50;
                    let dreadd_modulator = base_val_dq.powf(n_dq);

                    // --- J[0, j] (Derivative of brain RMA, species[0]) ---
                    // f_0 depends on species[0] (linearly), species[2], species[3]

                    // J[0, 0] = df0 / dy0
                    let j00 = -self.rma_config.rt_rate;

                    // Common term: df0 / dM_dox, where M_dox = ta_modulator
                    let d_f0_d_m_dox = (self.rma_config.prod_rate
                        - self.rma_config.leaky_prod_rate)
                        / (1. + ta_modulator).powi(2);

                    // J[0, 2] = (df0 / dM_dox) * (dM_dox / dy2)
                    // M_dox = (B_dox * y2)^n_dox; B_dox = active_tta_frac_term / self.tta_config.tta_kd
                    let d_base_dox_dy2 = active_tta_frac_term / self.tta_config.tta_kd;
                    let dm_dox_dy2 = if n_dox == 1 {
                        d_base_dox_dy2
                    } else {
                        n_dox_f * base_val_dox.powi(n_dox - 1) * d_base_dox_dy2
                    };
                    let j02 = d_f0_d_m_dox * dm_dox_dy2;

                    // J[0, 3] = (df0 / dM_dox) * (dM_dox / dy3)
                    // d(active_tta_frac_term) / dy3 = -1/(kd_dox + y3)^2 * kd_dox
                    let d_active_tta_frac_dy3 = -kd_dox / (kd_dox + species[3]).powi(2);

                    // dM_dox / dy3 = n_dox * M_dox^(n_dox-1) * (d_base_dox_dy3)
                    let d_base_dox_dy3 =
                        d_active_tta_frac_dy3 * species[2] / self.tta_config.tta_kd;
                    let dm_dox_dy3 = if n_dox == 1 {
                        d_base_dox_dy3
                    } else {
                        n_dox_f * base_val_dox.powi(n_dox - 1) * d_base_dox_dy3
                    };
                    let j03 = d_f0_d_m_dox * dm_dox_dy3;

                    // --- J[1, j] (Derivative of plasma RMA, species[1]) ---
                    // f_1 depends on species[0], species[1] (linearly)
                    let j10 = self.rma_config.rt_rate;
                    let j11 = -self.rma_config.deg_rate;

                    // --- J[2, j] (Derivative of tTA, species[2]) ---
                    // f_2 depends on species[2] (linearly), species[5], species[7], species[9]

                    // J[2, 2] = df2 / dy2
                    let j22 = -self.tta_config.deg_rate;

                    // Common term: df2 / dM_dq, where M_dq = dreadd_modulator
                    let d_f2_d_m_dq = (self.tta_config.prod_rate - self.tta_config.leaky_prod_rate)
                        / (1. + dreadd_modulator).powi(2);

                    // J[2, 5] = (df2 / dM_dq) * (dM_dq / dy5)
                    // M_dq = (B_dq * y5)^n_dq; B_dq = active_dread_frac / self.dq_config.ec50
                    let d_base_dq_dy5 = active_dread_frac / self.dq_config.ec50;
                    let dm_dq_dy5 = n_dq * base_val_dq.powf(n_dq - 1.) * d_base_dq_dy5;
                    let j25 = d_f2_d_m_dq * dm_dq_dy5;

                    // J[2, 7] = (df2 / dM_dq) * (dM_dq / dy7)
                    // J[2, 9] = (df2 / dM_dq) * (dM_dq / dy9)
                    // This is the most complex part: dM_dq / dy_i = (dM_dq / d(act_dread_frac)) * (d(act_dread_frac) / dy_i)

                    // d(act_dread_frac) / d(cno_hill) = (1. + clz_hill) / (1. + cno_hill + clz_hill)^2
                    let d_act_dread_d_cno_hill = (1. + clz_ec50_hill) / denominator_dread.powi(2);
                    // d(act_dread_frac) / d(clz_hill) = (1. + cno_hill) / (1. + cno_hill + clz_hill)^2
                    let d_act_dread_d_clz_hill = (1. + cno_ec50_hill) / denominator_dread.powi(2);

                    // d(cno_hill) / dy7:
                    // cno_hill = (A_cno * y7)^n_cno; A_cno = 1 / (cno_vd * cno_ec50)
                    let d_cno_hill_dy7 = n_cno * cno_ec50_hill / species[7]; // n_cno * A_cno^n_cno * y7^(n_cno-1) * A_cno
                                                                             // d(clz_hill) / dy9:
                                                                             // clz_hill = (A_clz * y9)^n_clz; A_clz = 1 / (clz_vd * clz_ec50)
                    let d_clz_hill_dy9 = n_clz * clz_ec50_hill / species[9];

                    // dM_dq / d(act_dread_frac)
                    // M_dq = ((act_dread) * B_dq_const)^n_dq; B_dq_const = y5 / self.dq_config.ec50
                    let b_dq_const = species[5] / self.dq_config.ec50;
                    let dm_dq_d_act =
                        n_dq * b_dq_const.powf(n_dq) * active_dread_frac.powf(n_dq - 1.);

                    // J[2, 7]
                    let j27 = d_f2_d_m_dq * dm_dq_d_act * d_act_dread_d_cno_hill * d_cno_hill_dy7;

                    // J[2, 9]
                    let j29 = d_f2_d_m_dq * dm_dq_d_act * d_act_dread_d_clz_hill * d_clz_hill_dy9;

                    // --- J[3, j] (Derivative of brain DOX, species[3]) ---
                    // f_3 depends on species[3], species[4] (linearly)
                    let j33 = -self.dox_config.plasma_transport_rate;
                    let j34 = self.dox_config.brain_transport_rate;

                    // --- J[4, j] (Derivative of plasma DOX, species[4]) ---
                    // f_4 depends on species[3], species[4] (linearly)
                    let j43 = self.dox_config.plasma_transport_rate;
                    let j44 =
                        -(self.dox_config.elimination_rate + self.dox_config.brain_transport_rate);

                    // --- J[5, j] (Derivative of DREADD, species[5]) ---
                    // f_5 depends on species[5] (linearly)
                    let j55 = -self.dq_config.deg_rate;

                    // --- J[6, j] (Derivative of peritoneal CNO, species[6]) ---
                    // f_6 depends on species[6] (linearly)
                    let j66 = -self.cno_config.cno_absorption_rate;

                    // --- J[7, j] (Derivative of brain CNO, species[7]) ---
                    // f_7 depends on species[7], species[8] (linearly)
                    let j77 = -self.cno_config.cno_plasma_transport_rate;
                    let j78 = self.cno_config.cno_brain_transport_rate;

                    // --- J[8, j] (Derivative of plasma CNO, species[8]) ---
                    // f_8 depends on species[6], species[7], species[8], species[10] (linearly)
                    let j86 = self.cno_config.cno_absorption_rate;
                    let j87 = self.cno_config.cno_plasma_transport_rate;
                    let j88 = -(self.cno_config.cno_elimination_rate
                        + self.cno_config.cno_brain_transport_rate
                        + self.cno_config.cno_reverse_metabolism_rate);
                    let j810 = self.cno_config.clz_metabolism_rate;

                    // --- J[9, j] (Derivative of brain CLZ, species[9]) ---
                    // f_9 depends on species[9], species[10] (linearly)
                    let j99 = -self.cno_config.clz_plasma_transport_rate;
                    let j910 = self.cno_config.clz_brain_transport_rate;

                    // --- J[10, j] (Derivative of plasma CLZ, species[10]) ---
                    // f_10 depends on species[8], species[9], species[10] (linearly)
                    let j10_8 = self.cno_config.cno_reverse_metabolism_rate;
                    let j10_9 = self.cno_config.clz_plasma_transport_rate;
                    let j10_10 = -(self.cno_config.clz_metabolism_rate
                        + self.cno_config.clz_elimination_rate
                        + self.cno_config.clz_brain_transport_rate);

                    // --- Calculate the Jacobian-vector product: diff = J * jac_vec ---
                    // diff[i] = Sum_{j=0}^{10} J[i, j] * jac_vec[j]

                    // diff[0] (Brain RMA)
                    diff[0] = (j00 * jac_vec[0]) + (j02 * jac_vec[2]) + (j03 * jac_vec[3]);

                    // diff[1] (Plasma RMA)
                    diff[1] = (j10 * jac_vec[0]) + (j11 * jac_vec[1]);

                    // diff[2] (tTA)
                    diff[2] = (j22 * jac_vec[2])
                        + (j25 * jac_vec[5])
                        + (j27 * jac_vec[7])
                        + (j29 * jac_vec[9]);

                    // diff[3] (Brain DOX)
                    diff[3] = (j33 * jac_vec[3]) + (j34 * jac_vec[4]);

                    // diff[4] (Plasma DOX)
                    diff[4] = (j43 * jac_vec[3]) + (j44 * jac_vec[4]);

                    // diff[5] (hM3Dq)
                    diff[5] = j55 * jac_vec[5];

                    // diff[6] (Peritoneal CNO)
                    diff[6] = j66 * jac_vec[6];

                    // diff[7] (Brain CNO)
                    diff[7] = (j77 * jac_vec[7]) + (j78 * jac_vec[8]);

                    // diff[8] (Plasma CNO)
                    diff[8] = (j86 * jac_vec[6])
                        + (j87 * jac_vec[7])
                        + (j88 * jac_vec[8])
                        + (j810 * jac_vec[10]);

                    // diff[9] (Brain CLZ)
                    diff[9] = (j99 * jac_vec[9]) + (j910 * jac_vec[10]);

                    // diff[10] (Plasma CLZ)
                    diff[10] = (j10_8 * jac_vec[8]) + (j10_9 * jac_vec[9]) + (j10_10 * jac_vec[10]);
                },
            )
            .init(
                |_p, _t, diff| {
                    init.iter().enumerate().for_each(|(idx, value)| {
                        diff[idx] = *value;
                    });
                },
                self.n_states,
            )
            .build()
            .unwrap();

        let n_steps = tf as usize;
        let times: Vec<f64> = ndarray::linspace(0., tf, n_steps).into_iter().collect();

        let mut solver = problem.bdf::<LinearSolver>().unwrap();

        // run first segment

        // let solution = match solver_type {
        //     SolverType::Tsit45 => {
        //         let mut solver = problem.tsit45().unwrap();
        //         solver.solve_dense(&times).unwrap()
        //     }
        //     SolverType::Bdf => {
        //         let mut solver = problem.bdf::<LinearSolver>().unwrap();
        //         solver.solve_dense(&times).unwrap()
        //     }
        // };

        Solution::new(times, solution.inner().to_owned(), ModelType::TetOff)
    }
}
