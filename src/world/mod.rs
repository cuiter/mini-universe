pub mod agent;
pub mod plant_grid;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::util::{Size2i, Vec2f, vec2f_to_vec2i};

pub struct World {
    pub agents: Vec<agent::Agent>,
    pub plant_grid: plant_grid::PlantGrid
}

impl World {
    pub fn new(plant_grid_size: Size2i) -> World {
        let time_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seed = time_since_epoch.as_millis();

        let mut plant_grid = plant_grid::PlantGrid::new(plant_grid_size, seed as u64);
        plant_grid.generate();
        World {
            agents: vec![agent::Agent::new(Vec2f::new(10.0, 10.0), 0.2)],
            plant_grid
        }
    }

    pub fn tick(&mut self, d_time: f32) {
        self.plant_grid.tick(d_time);

        for agent in self.agents.iter_mut() {
            if agent.tick(&self.plant_grid, d_time) {
                self.plant_grid.set_density(vec2f_to_vec2i(agent.get_mouth_pos()), 0);
            }
        }
    }
}
