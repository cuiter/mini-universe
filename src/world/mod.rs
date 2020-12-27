pub mod agent;
pub mod plant_grid;

use crate::util::{Size2i, Vec2f};

pub struct World {
    pub agents: Vec<agent::Agent>,
    pub plant_grid: plant_grid::PlantGrid
}

impl World {
    pub fn new(plant_grid_size: Size2i) -> World {
        let mut plant_grid = plant_grid::PlantGrid::new(plant_grid_size, 0);
        plant_grid.generate();
        World {
            agents: vec![agent::Agent::new(Vec2f::new(10.0, 10.0))],
            plant_grid
        }
    }

    pub fn tick(&mut self, d_time: f32) {
        for agent in self.agents.iter_mut() {
            agent.tick(d_time);
        }
        self.plant_grid.tick(d_time);
    }
}
