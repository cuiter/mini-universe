use crate::util::{time_ns, vec2f_to_vec2i, WRng};
use crate::world::{Agent, Params, PlantGrid};
use rand::SeedableRng;

/// A universe in which everything resides.
/// Contains a plant grid and a number of agents.
/// Keeps track of the current (simulation) time and various agent records.
pub struct World {
    pub agents: Vec<Agent>,
    pub plant_grid: PlantGrid,
    pub time: f64, // 64-bit required for precision after ~1 million seconds.
    pub seed: u64,
    rng: WRng,
    max_time_alive: f32,
    max_generation: u32,
}

impl World {
    /// Generate a new world with the specified parameters.
    /// If a seed is given, the same parameters will always yield the same result.
    /// If no seed is given, one will be generated.
    pub fn new(params: &Params) -> World {
        let seed = match params.seed {
            Some(seed) => seed,
            None => {
                let seed = time_ns() as u64 % 10_000_000;
                println!("using seed: {}", seed);
                seed
            }
        };

        World::new_seeded(params, seed)
    }

    /// Generate a new world with the specified parameters,
    /// using the given seed instead of the one from `params`.
    pub fn new_seeded(params: &Params, seed: u64) -> World {
        let mut rng = WRng::seed_from_u64(seed);

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
            seed,
            rng,
            max_time_alive: 0.0,
            max_generation: 1,
        }
    }

    /// Run the world for the specified amount of time.
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
                            "[{}] new time alive record: {}",
                            self.time.floor(),
                            self.max_time_alive
                        );
                    }
                    if agent.generation > self.max_generation {
                        self.max_generation = agent.generation;
                        println!(
                            "[{}] new generation record: {}",
                            self.time.floor(),
                            self.max_generation
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

        self.time += d_time as f64;
    }
}
