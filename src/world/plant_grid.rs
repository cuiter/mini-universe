use crate::util::{Size2i, Vec2i};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

const GENERATE_DENSITY_THRESHOLD: f32 = 0.999;
const GENERATE_REGENERATE: u32 = 100;
const REGENERATE_INTERVAL: f32 = 100.0;
const REGENERATE_NEIGHBOR_THRESHOLD: f32 = 100.0;
const REGENERATE_INCREMENT_MAX: f32 = 3.0;

#[derive(Clone)]
pub struct PlantGrid {
    pub densities: Vec<u8>,
    pub size: Size2i,
    rng: Pcg32,
    time_since_regenerate: f32
}

impl PlantGrid {
    pub fn new(size: Size2i, random_seed: u64) -> PlantGrid {
        PlantGrid {
            densities: vec![0u8; size.w as usize * size.h as usize],
            size,
            rng: Pcg32::seed_from_u64(random_seed),
            time_since_regenerate: 0.0
        }
    }

    #[inline]
    pub fn get_density(&self, pos: Vec2i) -> u8 {
        if pos.x < 0 || pos.y < 0 || pos.x as u32 >= self.size.w || pos.y as u32 >= self.size.h {
            0
        } else {
            self.get_density_unchecked(pos)
        }
    }

    #[inline]
    pub fn get_density_unchecked(&self, pos: Vec2i) -> u8 {
        self.densities[pos.y as usize * self.size.w as usize + pos.x as usize]
    }

    #[inline]
    pub fn set_density(&mut self, pos: Vec2i, density: u8) {
        self.densities[pos.y as usize * self.size.w as usize + pos.x as usize] = density;
    }

    pub fn generate(&mut self) {
        for row in 0..self.size.h {
            for col in 0..self.size.w {
                let random_value = self.rng.gen::<f32>();
                if random_value > GENERATE_DENSITY_THRESHOLD {
                    let new_density = (((random_value - GENERATE_DENSITY_THRESHOLD) / (1.0 - GENERATE_DENSITY_THRESHOLD)) * 255.0) as u8;
                    self.set_density(Vec2i::new(row as i32, col as i32), new_density);
                }
            }
        }

        for i in 0..GENERATE_REGENERATE {
            self.regenerate();
        }
    }

    pub fn tick(&mut self, d_time: f32) {
        self.time_since_regenerate += d_time;
        while self.time_since_regenerate > REGENERATE_INTERVAL {
            self.regenerate();
            self.time_since_regenerate -= REGENERATE_INTERVAL;
        }
    }

    fn regenerate(&mut self) {
        for row in 0..self.size.h {
            for col in 0..self.size.w {
                let mut neighbor_total = 0.0f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 + 1, row as i32 + 0)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 + 1, row as i32 + 1)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 + 0, row as i32 + 1)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 - 1, row as i32 + 1)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 - 1, row as i32 + 0)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 - 1, row as i32 - 1)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 + 0, row as i32 - 1)) as f32;
                neighbor_total += self.get_density(Vec2i::new(col as i32 + 1, row as i32 - 1)) as f32;

                let pos = Vec2i::new(col as i32, row as i32);
                if neighbor_total > REGENERATE_NEIGHBOR_THRESHOLD {
                    let mut new_density = self.get_density(pos) as f32;
                    new_density += REGENERATE_INCREMENT_MAX * self.rng.gen::<f32>();
                    if new_density > 255.0 {
                        new_density = 255.0;
                    }
                    self.set_density(pos, new_density as u8);
                }
            }
        }
    }
}
