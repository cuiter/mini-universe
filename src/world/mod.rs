pub mod agent;
pub mod plant_grid;
pub mod params;

pub use params::Params;

use std::time::{SystemTime, UNIX_EPOCH};
use crate::util::{WRng, Vec2f, vec2f_to_vec2i};
use rand::{Rng, SeedableRng};

pub struct World {
    pub agents: Vec<agent::Agent>,
    pub plant_grid: plant_grid::PlantGrid,
    rng: WRng,
}

impl World {
    pub fn new(params: &Params) -> World {
        let mut rng = match params.seed {
            Some(seed) => {
                WRng::seed_from_u64(seed)
            },
            None => {
                let time_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                let seed = time_since_epoch.as_millis() as u64;
                println!("using seed: {}", seed);
                WRng::seed_from_u64(seed)
            }
        };

        let mut plant_grid = plant_grid::PlantGrid::new(params.plant_grid_size);
        plant_grid.generate(&mut rng);
        World {
            agents: vec![agent::Agent::new(Vec2f::new(10.0, 10.0), 0.2)],
            plant_grid,
            rng
        }
    }

    pub fn tick(&mut self, d_time: f32) {
        self.plant_grid.tick(d_time, &mut self.rng);

        for agent in self.agents.iter_mut() {
            if agent.tick(&self.plant_grid, d_time) {
                self.plant_grid.set_density(vec2f_to_vec2i(agent.get_mouth_pos()), 0);
            }
        }
    }
}
