use crate::util::{WRng};
use rand::Rng;
use vek::ops::Clamp;

pub const N_PERCEPTS: usize = 4;
pub const N_COMMANDS: usize = 2;
const MUTATION_FACTOR: f32 = 0.02;

pub struct Brain {
    weights: [f32; N_PERCEPTS * N_COMMANDS]
}

impl Brain {
    pub fn new_random(rng: &mut WRng) -> Brain {
        let mut weights = [0.0; N_PERCEPTS * N_COMMANDS];

        for row in 0..N_PERCEPTS {
            for col in 0..N_COMMANDS {
                if row == 0 {
                    weights[row * N_COMMANDS + col] = rng.gen::<f32>();
                } else {
                    weights[row * N_COMMANDS + col] = rng.gen::<f32>() * 2.0 - 1.0;
                }
            }
        }

        Brain {
            weights
        }
    }

    pub fn reproduce(&self, rng: &mut WRng) -> Brain {
        let mut new_weights = self.weights.clone();
        for row in 0..N_PERCEPTS {
            for col in 0..N_COMMANDS {
                let weight = new_weights[row * N_COMMANDS + col];
                let new_weight = weight + (rng.gen::<f32>() * 2.0 - 1.0) * MUTATION_FACTOR;
                let clamped_new_weight = if row == 0 { new_weight.clamped(0.0, 1.0) } else { new_weight.clamped(-1.0, 1.0) };
                new_weights[row * N_COMMANDS + col] = clamped_new_weight;
            }
        }
        Brain {
            weights: new_weights
        }
    }

    pub fn run(&self, percepts: [f32; N_PERCEPTS]) -> [f32; N_COMMANDS] {
        let mut result = [0.0; N_COMMANDS];

        for row in 0..N_PERCEPTS {
            for col in 0..N_COMMANDS {
                result[col] += percepts[row] * self.weights[row * N_COMMANDS + col];
            }
        }

        for i in 0..result.len() {
            result[i] = result[i].clamped(0.0, 1.0);
        }

        result
    }
}
