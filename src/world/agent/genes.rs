use crate::util::{Vec3f, WRng};
use rand::Rng;
use strum::EnumCount;
use strum_macros::{AsRefStr, EnumCount as EnumCountMacro, EnumIter};
use vek::ops::Lerp;

#[derive(Copy, Clone, PartialEq, EnumCountMacro, EnumIter, AsRefStr)]
pub enum Gene {
    MutationFactor = 0,
    ColorR = 1,
    ColorG = 2,
    ColorB = 3,
    Size = 4,
    Speed = 5,
    EyeDistance = 6,
    EyeAngle = 7,
    TimerInterval = 8,
}

const MAX_MUTATION_FACTOR: f32 = 0.1;
const MIN_SIZE: f32 = 2.5;
const MAX_SIZE: f32 = 5.0;
const MIN_SPEED: f32 = 5.0;
const MAX_SPEED: f32 = 10.0;
const MIN_EYE_DISTANCE: f32 = 4.0;
const MAX_EYE_DISTANCE: f32 = 10.0;
const MIN_EYE_ANGLE: f32 = 0.1;
const MAX_EYE_ANGLE: f32 = 0.5;
const MIN_TIMER_INTERVAL: f32 = 1.0;
const MAX_TIMER_INTERVAL: f32 = 50.0;

/// Struct that keeps track of an agent's genes.
/// Has methods for determining agent attributes.
pub struct Genes {
    genes: [f32; Gene::COUNT],
}

impl Genes {
    /// Generates random genes.
    pub fn new_random(rng: &mut WRng) -> Genes {
        let mut genes = [0.0; Gene::COUNT];

        for i in 0..Gene::COUNT {
            genes[i] = rng.gen::<f32>();
        }

        Genes { genes }
    }

    /// Reproduces the genes asexually, mutating according to the mutation factor.
    pub fn reproduce(&self, rng: &mut WRng) -> Genes {
        let mut new_genes = self.genes.clone();
        for i in 0..Gene::COUNT {
            new_genes[i] += (rng.gen::<f32>() * 2.0 - 1.0) * self.get_mutation_factor();
        }

        Genes { genes: new_genes }
    }

    pub fn get_mutation_factor(&self) -> f32 {
        self.genes[Gene::MutationFactor as usize] * MAX_MUTATION_FACTOR
    }

    pub fn get_color(&self) -> Vec3f {
        Vec3f::new(
            self.genes[Gene::ColorR as usize],
            self.genes[Gene::ColorG as usize],
            self.genes[Gene::ColorB as usize],
        )
    }

    pub fn get_size(&self) -> f32 {
        Lerp::lerp(MIN_SIZE, MAX_SIZE, self.genes[Gene::Size as usize])
    }
    pub fn get_speed(&self) -> f32 {
        Lerp::lerp(MIN_SPEED, MAX_SPEED, self.genes[Gene::Speed as usize])
    }
    pub fn get_eye_distance(&self) -> f32 {
        Lerp::lerp(
            MIN_EYE_DISTANCE,
            MAX_EYE_DISTANCE,
            self.genes[Gene::EyeDistance as usize],
        )
    }
    pub fn get_eye_angle(&self) -> f32 {
        Lerp::lerp(
            MIN_EYE_ANGLE,
            MAX_EYE_ANGLE,
            self.genes[Gene::EyeAngle as usize],
        )
    }
    pub fn get_timer_interval(&self) -> f32 {
        Lerp::lerp(
            MIN_TIMER_INTERVAL,
            MAX_TIMER_INTERVAL,
            self.genes[Gene::TimerInterval as usize],
        )
    }
}
