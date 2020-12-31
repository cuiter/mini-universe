use crate::util::{Vec3f, WRng};
use rand::Rng;
use vek::ops::Lerp;

const N_GENES: usize = 9;

const GENE_MUTATION_FACTOR: usize = 0;
const GENE_COLOR_R: usize = 1;
const GENE_COLOR_G: usize = 2;
const GENE_COLOR_B: usize = 3;
const GENE_SIZE: usize = 4;
const GENE_SPEED: usize = 5;
const GENE_EYE_DISTANCE: usize = 6;
const GENE_EYE_ANGLE: usize = 7;
const GENE_TIMER_INTERVAL: usize = 8;

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
    genes: [f32; N_GENES],
}

impl Genes {
    /// Generates random genes.
    pub fn new_random(rng: &mut WRng) -> Genes {
        let mut genes = [0.0; N_GENES];

        for i in 0..N_GENES {
            genes[i] = rng.gen::<f32>();
        }

        Genes { genes }
    }

    /// Reproduces the genes asexually, mutating according to the mutation factor.
    pub fn reproduce(&self, rng: &mut WRng) -> Genes {
        let mut new_genes = self.genes.clone();
        for i in 0..N_GENES {
            new_genes[i] += (rng.gen::<f32>() * 2.0 - 1.0) * self.get_mutation_factor();
        }

        Genes { genes: new_genes }
    }

    pub fn get_mutation_factor(&self) -> f32 {
        self.genes[GENE_MUTATION_FACTOR] * MAX_MUTATION_FACTOR
    }

    pub fn get_color(&self) -> Vec3f {
        Vec3f::new(
            self.genes[GENE_COLOR_R],
            self.genes[GENE_COLOR_G],
            self.genes[GENE_COLOR_B],
        )
    }

    pub fn get_size(&self) -> f32 {
        Lerp::lerp(MIN_SIZE, MAX_SIZE, self.genes[GENE_SIZE])
    }
    pub fn get_speed(&self) -> f32 {
        Lerp::lerp(MIN_SPEED, MAX_SPEED, self.genes[GENE_SPEED])
    }
    pub fn get_eye_distance(&self) -> f32 {
        Lerp::lerp(
            MIN_EYE_DISTANCE,
            MAX_EYE_DISTANCE,
            self.genes[GENE_EYE_DISTANCE],
        )
    }
    pub fn get_eye_angle(&self) -> f32 {
        Lerp::lerp(MIN_EYE_ANGLE, MAX_EYE_ANGLE, self.genes[GENE_EYE_ANGLE])
    }
    pub fn get_timer_interval(&self) -> f32 {
        Lerp::lerp(
            MIN_TIMER_INTERVAL,
            MAX_TIMER_INTERVAL,
            self.genes[GENE_TIMER_INTERVAL],
        )
    }
}
