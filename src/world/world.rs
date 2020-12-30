use crate::util::{vec2f_to_vec2i, WRng};
use crate::world::{Agent, Params, PlantGrid};
use rand::SeedableRng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct World {
    pub agents: Vec<Agent>,
    pub plant_grid: PlantGrid,
    pub time: f32,
    rng: WRng,
    max_time_alive: f32,
    max_generation: u32,
}

impl World {
    pub fn new(params: &Params) -> World {
        let mut rng = match params.seed {
            Some(seed) => WRng::seed_from_u64(seed),
            None => {
                let time_since_epoch = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let seed = time_since_epoch.as_millis() as u64;
                println!("using seed: {}", seed);
                WRng::seed_from_u64(seed)
            }
        };

        let mut plant_grid = PlantGrid::new(params.plant_grid_size);
        plant_grid.generate(&mut rng);
        let mut agents = Vec::with_capacity(params.agent_count as usize);
        for _ in 0..params.agent_count {
            agents.push(Agent::new_random(&params, &mut rng));
        }
        World {
            agents: agents,
            plant_grid,
            time: 0.0,
            rng,
            max_time_alive: 0.0,
            max_generation: 1,
        }
    }

    pub fn tick(&mut self, params: &Params, d_time: f32) {
        self.plant_grid.tick(d_time, &mut self.rng);

        let mut idx = 0;
        while idx < self.agents.len() {
            let agent = &mut self.agents[idx];
            let tick_result = agent.tick(&self.plant_grid, d_time);
            if tick_result.eat {
                self.plant_grid
                    .set_density(vec2f_to_vec2i(agent.get_mouth_pos()), 0);
            }

            if params.evolution {
                if tick_result.die {
                    if agent.time_alive > self.max_time_alive {
                        self.max_time_alive = agent.time_alive;
                        println!(
                            "[{:.0}] new time alive record: {}",
                            self.time, self.max_time_alive
                        );
                    }
                    if agent.generation > self.max_generation {
                        self.max_generation = agent.generation;
                        println!(
                            "[{:.0}] new generation record: {}",
                            self.time, self.max_generation
                        );
                    }

                    self.agents.remove(idx);
                    idx -= 1;

                    if self.agents.len() < params.agent_count as usize {
                        self.agents.push(Agent::new_random(&params, &mut self.rng));
                    }
                } else if tick_result.reproduce {
                    let new_agent = agent.reproduce(&mut self.rng);
                    self.agents.push(new_agent);
                }
            }

            idx += 1;
        }

        self.time += d_time;
    }
}
