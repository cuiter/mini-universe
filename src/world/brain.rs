use crate::util::{WRng};
use rand::Rng;
use vek::ops::Clamp;

pub const N_PERCEPTS: usize = 3;
pub const N_COMMANDS: usize = 2;

pub struct Brain {
    weights: [f32; N_PERCEPTS * N_COMMANDS]
}

impl Brain {
    pub fn new_random(rng: &mut WRng) -> Brain {
        let mut weights = [0.0; N_PERCEPTS * N_COMMANDS];
        for i in 0..weights.len() {
            weights[i] = rng.gen::<f32>() * 2.0 - 1.0;
        }

        Brain {
            weights
        }
    }

    pub fn reproduce(&self, rng: &mut WRng) -> Brain {
        Brain {
            weights: self.weights.clone()
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
