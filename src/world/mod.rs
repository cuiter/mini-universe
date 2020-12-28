pub mod agent;
pub mod plant_grid;
pub mod params;
mod brain;

pub use params::Params;

use std::time::{SystemTime, UNIX_EPOCH};
use crate::util::{WRng, Vec2f, vec2f_to_vec2i};
use rand::{Rng, SeedableRng};

pub struct World {
    pub agents: Vec<agent::Agent>,
    pub plant_grid: plant_grid::PlantGrid,
    rng: WRng,
    max_time_alive: f32,
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
        let mut agents = Vec::with_capacity(params.agent_count as usize);
        for i in 0..params.agent_count {
            agents.push(agent::Agent::new_random(&params, &mut rng));
        }
        World {
            agents: agents,
            plant_grid,
            rng,
            max_time_alive: 0.0
        }
    }

    pub fn tick(&mut self, params: &Params, d_time: f32) {
        self.plant_grid.tick(d_time, &mut self.rng);

        let mut idx = 0;
        while idx < self.agents.len() {
            let mut agent = &mut self.agents[idx];
            let tick_result = agent.tick(&self.plant_grid, d_time);
            if tick_result.eat {
                self.plant_grid.set_density(vec2f_to_vec2i(agent.get_mouth_pos()), 0);
            }

            if params.evolution {
                if tick_result.die {
                    if agent.time_alive > self.max_time_alive {
                        println!("new time alive record: {}", agent.time_alive);
                        self.max_time_alive = agent.time_alive;
                    }

                    self.agents.remove(idx);
                    idx -= 1;

                    if self.agents.len() < params.agent_count as usize {
                        self.agents.push(agent::Agent::new_random(&params, &mut self.rng));
                    }
                } else if tick_result.reproduce {
                    let new_agent = agent.reproduce(&mut self.rng);
                    self.agents.push(new_agent);
                }
            }

            idx += 1;
        }
    }
}
