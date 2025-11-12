use crate::{ModelType, Solution, SolverType};
use diffsol::{MatrixCommon, NalgebraLU, NalgebraMat, OdeBuilder, OdeSolverMethod};

type LinearSolver = NalgebraLU<f64>;

pub struct ConstitutiveRMA {
    prod_rate: f64,
    rt_rate: f64,
    deg_rate: f64,
    n_states: usize,
}

impl ConstitutiveRMA {
    /// Create new model of constitutive RMA expression for a given
    /// production, reverse transcytosis, and degradation rate.
    pub fn new(prod_rate: f64, rt_rate: f64, deg_rate: f64) -> Self {
        Self {
            prod_rate,
            rt_rate,
            deg_rate,
            n_states: 2,
        }
    }

    pub fn solve(&self, tf: f64, init: Vec<f64>, solver_type: SolverType) -> Solution {
        let problem = OdeBuilder::<NalgebraMat<f64>>::new()
            .p(vec![self.prod_rate, self.rt_rate, self.deg_rate])
            .rhs_implicit(
                |species, params, _t, diff| {
                    let brain_efflux = params[1] * species[0];
                    let degradation = params[2] * species[1];

                    diff[0] = params[0] - brain_efflux;
                    diff[1] = brain_efflux - degradation;
                },
                |_species, params, _t, jac_vec, diff| {
                    diff[0] = -params[1] * jac_vec[0];
                    diff[1] = params[1] * jac_vec[0] - params[2] * jac_vec[1];
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

        Solution::new(times, solution.inner().to_owned(), ModelType::Constitutive)
    }
}
