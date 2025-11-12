use crate::ModelType;
use diffsol::NalgebraLU;
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};

type LinearSolver = NalgebraLU<f64>;

#[derive(Serialize, Deserialize)]
pub struct Solution {
    pub ts: Vec<f64>,
    pub ys: DMatrix<f64>,
    pub model: ModelType,
}

impl Solution {
    pub fn new(ts: Vec<f64>, ys: DMatrix<f64>, model: ModelType) -> Self {
        Self { ts, ys, model }
    }
}

pub enum SolverType {
    Tsit45,
    Bdf,
}

// pub fn solve(
//     problem: OdeSolverProblem<
//         impl OdeEquations<T = f64, V = NalgebraVec<f64>, M = NalgebraMat<f64>>,
//     >,
//     solver_type: SolverType,
//     model_type: ModelType,
//     tf: f64,
//     sampling_rate: f64,
// ) -> Solution {
//     let n_steps = (tf * sampling_rate) as usize;
//     let times: Vec<f64> = ndarray::linspace(0., tf, n_steps).into_iter().collect();
//     let solution = match solver_type {
//         SolverType::Tsit45 => {
//             let mut solver = problem.tsit45().unwrap();
//             solver.solve_dense(&times).unwrap()
//         }
//         SolverType::Bdf => {
//             let mut solver = problem.bdf::<LinearSolver>().unwrap();
//             solver.solve_dense(&times).unwrap()
//         }
//     };

//     Solution::new(times, solution.inner().to_owned(), model_type)
// }
