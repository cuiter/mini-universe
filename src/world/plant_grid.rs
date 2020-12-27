use crate::util::{Size2i, Vec2i};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

#[derive(Clone)]
pub struct PlantGrid {
    pub densities: Vec<u8>,
    pub size: Size2i,
    rng: Pcg32
}

impl PlantGrid {
    pub fn new(size: Size2i, random_seed: u64) -> PlantGrid {
        PlantGrid {
            densities: vec![0u8; size.w as usize * size.h as usize],
            size,
            rng: Pcg32::seed_from_u64(random_seed)
        }
    }

    #[inline]
    pub fn get_density(&self, pos: Vec2i) -> u8 {
        self.densities[pos.y as usize * self.size.w as usize + pos.x as usize]
    }

    #[inline]
    fn set_density(&mut self, pos: Vec2i, density: u8) {
        self.densities[pos.y as usize * self.size.w as usize + pos.x as usize] = density;
    }

    pub fn generate(&mut self) {
        for row in 0..self.size.h {
            for col in 0..self.size.w {
                let new_density = self.rng.gen::<u8>();
                self.set_density(Vec2i::new(row as i32, col as i32), new_density);
            }
        }
    }

    pub fn tick(&mut self, d_time: f32) {
        // TODO: Implement plant growing
    }
}
