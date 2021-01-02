use crate::util::WRng;
use rand::Rng;
use strum::EnumCount;
use strum_macros::{AsRefStr, EnumCount as EnumCountMacro, EnumIter};
use vek::ops::Clamp;

#[derive(Copy, Clone, PartialEq, EnumCountMacro, EnumIter, AsRefStr)]
pub enum Percept {
    ConstantOne = 0,
    LeftEye = 1,
    RightEye = 2,
    TimeWave = 3,
}

#[derive(Copy, Clone, PartialEq, EnumCountMacro, EnumIter, AsRefStr)]
pub enum Command {
    LeftTrack = 0,
    RightTrack = 1,
}

pub type Percepts = [f32; Percept::COUNT];
pub type Commands = [f32; Command::COUNT];

/// A brain calculates what commands to send to the actuators,
/// based on the inputs from the sensors.
/// Currently uses a simple single-layer feed-forward neural network.
/// The activation function is an add (+) clamped to [0.0, 1.0].
pub struct Brain {
    weights: [f32; Percept::COUNT * Command::COUNT],
}

impl Brain {
    /// Generates a random brain.
    pub fn new_random(rng: &mut WRng) -> Brain {
        let mut weights = [0.0; Percept::COUNT * Command::COUNT];

        for row in 0..Percept::COUNT {
            for col in 0..Command::COUNT {
                if row == 0 {
                    weights[row * Command::COUNT + col] = rng.gen::<f32>();
                } else {
                    weights[row * Command::COUNT + col] = rng.gen::<f32>() * 2.0 - 1.0;
                }
            }
        }

        Brain { weights }
    }

    /// Reproduces the brain asexually, mutating according to the mutation factor.
    pub fn reproduce(&self, mutation_factor: f32, rng: &mut WRng) -> Brain {
        let mut new_weights = self.weights.clone();
        for row in 0..Percept::COUNT {
            for col in 0..Command::COUNT {
                let weight = new_weights[row * Command::COUNT + col];
                let new_weight = weight + (rng.gen::<f32>() * 2.0 - 1.0) * mutation_factor;
                let clamped_new_weight = if row == 0 {
                    new_weight.clamped(0.0, 1.0)
                } else {
                    new_weight.clamped(-1.0, 1.0)
                };
                new_weights[row * Command::COUNT + col] = clamped_new_weight;
            }
        }
        Brain {
            weights: new_weights,
        }
    }

    /// Use the brain to calculate what commands to send to the actuators based on the given
    /// percepts from the sensors.
    pub fn run(&self, percepts: &Percepts) -> Commands {
        let mut result = [0.0; Command::COUNT];

        for row in 0..Percept::COUNT {
            for col in 0..Command::COUNT {
                result[col] += percepts[row] * self.weights[row * Command::COUNT + col];
            }
        }

        for i in 0..result.len() {
            result[i] = result[i].clamped(0.0, 1.0);
        }

        result
    }
}
