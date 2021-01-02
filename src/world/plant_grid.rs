use crate::util::{Size2i, Vec2i, WRng};
use rand::Rng;

const GENERATE_DENSITY_THRESHOLD: f32 = 0.995;
const GENERATE_REGENERATE: u32 = 100;
const REGENERATE_INTERVAL: f32 = 10.0;
const REGENERATE_NEIGHBOR_THRESHOLD: f32 = 100.0;
const REGENERATE_INCREMENT_MAX: f32 = 3.0;

const TARGET_DENSITY_PER_CELL: f32 = 5.0;

#[derive(Clone)]
pub struct PlantGrid {
    pub densities: Vec<u8>,
    pub size: Size2i,
    time_since_regenerate: f32,
}

impl PlantGrid {
    pub fn new(size: Size2i) -> PlantGrid {
        PlantGrid {
            densities: vec![0u8; size.w as usize * size.h as usize],
            size,
            time_since_regenerate: 0.0,
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

    fn get_target_total_density(&self) -> u64 {
        let total_density = self.size.w as f32 * self.size.h as f32 * TARGET_DENSITY_PER_CELL;

        total_density as u64
    }

    pub fn generate(&mut self, rng: &mut WRng) {
        // Don't let plants grow on the borders for performance reasons.
        for row in 1..self.size.h-1 {
            for col in 1..self.size.w-1 {
                let random_value = rng.gen::<f32>();
                if random_value > GENERATE_DENSITY_THRESHOLD {
                    let new_density = (((random_value - GENERATE_DENSITY_THRESHOLD)
                        / (1.0 - GENERATE_DENSITY_THRESHOLD))
                        * 255.0) as u8;
                    self.set_density(Vec2i::new(row as i32, col as i32), new_density);
                }
            }
        }

        for _ in 0..GENERATE_REGENERATE {
            self.regenerate(rng);
        }
    }

    pub fn tick(&mut self, d_time: f32, rng: &mut WRng) {
        self.time_since_regenerate += d_time;
        while self.time_since_regenerate > REGENERATE_INTERVAL {
            self.regenerate(rng);
            self.time_since_regenerate -= REGENERATE_INTERVAL;
        }
    }

    fn regenerate(&mut self, rng: &mut WRng) {
        let mut total_density: u64 = 0;
        let mut first_iteration = true;
        while total_density < self.get_target_total_density() {
            total_density = 0;
            // Don't let plants grow on the borders for performance reasons.
            for row in 1..self.size.h-1 {
                for col in 1..self.size.w-1 {
                    let mut neighbor_total = 0.0f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 + 1, row as i32 + 0)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 + 1, row as i32 + 1)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 + 0, row as i32 + 1)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 - 1, row as i32 + 1)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 - 1, row as i32 + 0)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 - 1, row as i32 - 1)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 + 0, row as i32 - 1)) as f32;
                    neighbor_total +=
                        self.get_density_unchecked(Vec2i::new(col as i32 + 1, row as i32 - 1)) as f32;

                    let pos = Vec2i::new(col as i32, row as i32);
                    if !first_iteration && neighbor_total > REGENERATE_NEIGHBOR_THRESHOLD {
                        let mut new_density = self.get_density_unchecked(pos) as f32;
                        new_density += REGENERATE_INCREMENT_MAX * rng.gen::<f32>();
                        if new_density > 255.0 {
                            new_density = 255.0;
                        }
                        self.set_density(pos, new_density as u8);
                    }

                    total_density += self.get_density_unchecked(pos) as u64;
                }
            }
            first_iteration = false;
        }
    }
}
