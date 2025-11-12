use crate::{dox::DoxPKConfig, DoxArgs, ModelType, Solution, SolverType};
use diffsol::{MatrixCommon, NalgebraLU, NalgebraMat, OdeBuilder, OdeSolverMethod};
use serde::{Deserialize, Serialize};

type LinearSolver = NalgebraLU<f64>;

pub struct TetoffRMA {
    rma_config: RmaConfig,
    tta_config: TtaConfig,
    dox_config: DoxPKConfig,
    n_states: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RmaConfig {
    pub prod_rate: f64,
    pub leaky_prod_rate: f64,
    pub rt_rate: f64,
    pub deg_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TtaConfig {
    pub prod_rate: f64,
    pub leaky_prod_rate: f64,
    pub deg_rate: f64,
    pub tta_kd: f64,
    pub tta_coop: i32,
}

impl TetoffRMA {
    /// Create new model of Tet-gated RMA expression.
    pub fn new(rma_config: RmaConfig, tta_config: TtaConfig, dox_config: DoxArgs) -> Self {
        let full_dox_config = DoxPKConfig::new(dox_config);
        Self {
            rma_config,
            tta_config,
            dox_config: full_dox_config,
            n_states: 5,
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

                    diff[0] = ((self.rma_config.leaky_prod_rate
                        + (self.rma_config.prod_rate * ta_modulator))
                        / (1. + ta_modulator))
                        - brain_rma_efflux;

                    diff[1] = brain_rma_efflux - (self.rma_config.deg_rate * species[1]);

                    // constitutive tTA expression
                    diff[2] = self.tta_config.prod_rate - (self.tta_config.deg_rate * species[2]);

                    diff[3] = plasma_dox_efflux - brain_dox_efflux;
                    diff[4] = (self.dox_config.absorption_rate * dox_intake)
                        - (self.dox_config.elimination_rate * species[4])
                        - plasma_dox_efflux
                        + brain_dox_efflux;
                },
                |species, _params, _t, jac_vec, diff| {
                    let n_int = self.tta_config.tta_coop;
                    let n_f = n_int as f64;
                    let kd = self.dox_config.dox_kd.unwrap_or_default();

                    let active_tta_frac = kd / (kd + species[3]);
                    let base_val = active_tta_frac * species[2] / self.tta_config.tta_kd;
                    let ta_modulator = base_val.powi(n_int);

                    // --- Calculate non-zero partial derivatives for J[0, j] ---
                    // f_0 = (P_L + P_R * M) / (1 + M) - C_3 * y_0
                    // M = ta_modulator

                    // J[0, 0] = df0 / dy0
                    let j00 = -self.rma_config.rt_rate;

                    // Common term: df0 / dM
                    let df0_dm = (self.rma_config.prod_rate - self.rma_config.leaky_prod_rate)
                        / (1. + ta_modulator).powi(2);

                    // J[0, 2] = (df0 / dM) * (dM / dy2)
                    // dM / dy2 = d( (base_dy2 * y2)^n ) / dy2 = n * (base_dy2 * y2)^(n-1) * base_dy2
                    let d_base_dy2 = active_tta_frac / self.tta_config.tta_kd;
                    let dm_dy2 = if n_int == 1 {
                        d_base_dy2
                    } else {
                        n_f * base_val.powi(n_int - 1) * d_base_dy2
                    };
                    let j02 = df0_dm * dm_dy2;

                    // J[0, 3] = (df0 / dM) * (dM / dy3)
                    // dM / dy3 = -n * M / (kd + y3)
                    let dm_dy3 = -n_f * ta_modulator / (kd + species[3]);
                    let j03 = df0_dm * dm_dy3;

                    // --- Calculate the Jacobian-vector product: diff = J * jac_vec ---

                    // diff[0] = J[0,0]*v[0] + J[0,1]*v[1] + J[0,2]*v[2] + J[0,3]*v[3] + J[0,4]*v[4]
                    // J[0,1] and J[0,4] are 0
                    diff[0] = (j00 * jac_vec[0]) + (j02 * jac_vec[2]) + (j03 * jac_vec[3]);

                    // diff[1] = J[1,0]*v[0] + J[1,1]*v[1] + ...
                    // f1 = C3*y0 - C4*y1
                    // J[1,0] = self.rma_config.rt_rate
                    // J[1,1] = -self.rma_config.deg_rate
                    diff[1] = (self.rma_config.rt_rate * jac_vec[0])
                        - (self.rma_config.deg_rate * jac_vec[1]);

                    // diff[2] = J[2,2]*v[2]
                    // f2 = C5 - C6*y2
                    // J[2,2] = -self.tta_config.deg_rate
                    diff[2] = -self.tta_config.deg_rate * jac_vec[2];

                    // diff[3] = J[3,3]*v[3] + J[3,4]*v[4]
                    // f3 = C2*y4 - C1*y3
                    // J[3,3] = -self.dox_config.plasma_transport_rate
                    // J[3,4] = self.dox_config.brain_transport_rate
                    diff[3] = (-self.dox_config.plasma_transport_rate * jac_vec[3])
                        + (self.dox_config.brain_transport_rate * jac_vec[4]);

                    // diff[4] = J[4,3]*v[3] + J[4,4]*v[4]
                    // f4 = C7*I(t) - (C8 + C2)*y4 + C1*y3
                    // J[4,3] = self.dox_config.plasma_transport_rate
                    // J[4,4] = -(self.dox_config.elimination_rate + self.dox_config.brain_transport_rate)
                    diff[4] = (self.dox_config.plasma_transport_rate * jac_vec[3])
                        - ((self.dox_config.elimination_rate
                            + self.dox_config.brain_transport_rate)
                            * jac_vec[4]);
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

        let solution = match solver_type {
            SolverType::Tsit45 => {
                let mut solver = problem.tsit45().unwrap();
                solver.solve_dense(&times).unwrap()
            }
            SolverType::Bdf => {
                let mut solver = problem.bdf::<LinearSolver>().unwrap();
                solver.solve_dense(&times).unwrap()
            }
        };

        Solution::new(times, solution.inner().to_owned(), ModelType::TetOff)
    }
}
