use crate::util::{WRng, Vec2f, Vec3f, Rect2f, vec2f_to_vec2i, calculate_vec2f};
use crate::world::plant_grid::PlantGrid;
use crate::world::brain::{Brain, N_PERCEPTS, N_COMMANDS};
use crate::world::params::Params;
use vek::ops::Clamp;
use rand::Rng;

const DEFAULT_RADIUS: f32 = 2.0;
const MAX_SPEED: f32 = 8.0;
const MEASURE_DISTANCE: f32 = 8.0;
const MEASURE_ANGLE: f32 = 0.35;
const MOUTH_DISTANCE: f32 = 2.0;

const PERCEPT_LEFT_EYE: usize = 1;
const PERCEPT_RIGHT_EYE: usize = 2;

const ENERGY_LOSE_SPEED: f32 = 0.02;
const ENERGY_EAT_GAIN: f32 = 0.1;

const REPRODUCE_INTERVAL: f32 = 100.0;
const REPRODUCE_ENERGY_COST: f32 = 0.5;

pub struct Agent {
    pub radius: f32,
    pub color: Vec3f,
    pub pos: Vec2f,
    pub angle: f32, // radians
    pub energy: f32,
    pub time_alive: f32,
    pub time_since_reproduce: f32,
    brain: Brain,
}

pub struct TickResult {
    pub eat: bool,
    pub die: bool,
    pub reproduce: bool,
}

struct Sensors {
    pub percepts: [f32; N_PERCEPTS] // 0: constant one, 1: left eye, 2: right eye, 3: time (sinusoid)
}

struct Actuators {
    pub commands: [f32; N_COMMANDS] // 0: left speed, 1: right speed
}

impl Agent {
    pub fn new_random(params: &Params, rng: &mut WRng) -> Agent {
        let pos = Vec2f::new(rng.gen::<f32>() * params.plant_grid_size.w as f32, rng.gen::<f32>() * params.plant_grid_size.h as f32);
        let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
        Agent {
            radius: DEFAULT_RADIUS,
            color: Vec3f::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
            pos,
            angle,
            energy: 1.0,
            time_alive: 0.0,
            time_since_reproduce: 0.0,
            brain: Brain::new_random(rng),
        }
    }

    pub fn reproduce(&self, rng: &mut WRng) -> Agent {
        Agent {
            radius: self.radius,
            color: self.color,
            pos: self.pos,
            angle: -self.angle,
            energy: 1.0,
            time_alive: 0.0,
            time_since_reproduce: 0.0,
            brain: self.brain.reproduce(rng)
        }
    }

    #[inline]
    pub fn get_bounding_rect(&self) -> Rect2f {
        Rect2f::new(self.pos.x - self.radius, self.pos.y - self.radius, self.radius * 2.0, self.radius * 2.0)
    }

    pub fn get_left_measure_pos(&self) -> Vec2f {
        self.pos + calculate_vec2f(MEASURE_DISTANCE, self.angle - MEASURE_ANGLE)
    }
    pub fn get_right_measure_pos(&self) -> Vec2f {
        self.pos + calculate_vec2f(MEASURE_DISTANCE, self.angle + MEASURE_ANGLE)
    }
    pub fn get_mouth_pos(&self) -> Vec2f {
        self.pos + calculate_vec2f(MOUTH_DISTANCE, self.angle)
    }

    fn measure_sensors(&self, plant_grid: &PlantGrid) -> Sensors {
        let left_density = plant_grid.get_density(vec2f_to_vec2i(self.get_left_measure_pos()));
        let right_density = plant_grid.get_density(vec2f_to_vec2i(self.get_right_measure_pos()));

        Sensors {
            percepts: [
                1.0,
                left_density as f32 / 255.0,
                right_density as f32 / 255.0,
                self.time_alive.sin() // TODO: Add gene for changing period
            ]
        }
    }

    fn calculate_actuators(&self, sensors: &Sensors) -> Actuators {
        Actuators {
            commands: self.brain.run(sensors.percepts)
        }
    }

    fn apply_actuators(&mut self, actuators: &Actuators, plant_grid: &PlantGrid, d_time: f32) {
        let left_speed = (actuators.commands[0] - 0.5) * MAX_SPEED;
        let right_speed = (actuators.commands[1] - 0.5) * MAX_SPEED;

        let speed = left_speed + right_speed;
        let radial_speed = (1.0 / DEFAULT_RADIUS) * left_speed - (1.0 / DEFAULT_RADIUS) * right_speed;

        self.angle += radial_speed * d_time;
        self.pos += calculate_vec2f(speed, self.angle) * d_time;

        if self.pos.x < 0.0 {
            self.pos.x = plant_grid.size.w as f32;
        } else if self.pos.y < 0.0 {
            self.pos.y = plant_grid.size.h as f32;
        } else if self.pos.x > plant_grid.size.w as f32 {
            self.pos.x = 0.0
        } else if self.pos.y > plant_grid.size.h as f32 {
            self.pos.y = 0.0;
        }
    }

    pub fn tick(&mut self, plant_grid: &PlantGrid, d_time: f32) -> TickResult {
        let sensors = self.measure_sensors(plant_grid);
        let actuators = self.calculate_actuators(&sensors);
        self.apply_actuators(&actuators, plant_grid, d_time);

        let density_at_mouth = plant_grid.get_density(vec2f_to_vec2i(self.get_mouth_pos()));
        let eat = density_at_mouth > 0;

        self.energy -= ENERGY_LOSE_SPEED * d_time;
        if eat {
            self.energy += ENERGY_EAT_GAIN * (density_at_mouth as f32 / 255.0);
            self.energy = self.energy.clamped(0.0, 1.0);
        }

        self.time_alive += d_time;
        self.time_since_reproduce += d_time;

        let reproduce = self.time_since_reproduce > REPRODUCE_INTERVAL;
        if reproduce {
            self.time_since_reproduce = 0.0;
            self.energy -= REPRODUCE_ENERGY_COST;
        }

        TickResult {
            eat,
            die: self.energy <= 0.0,
            reproduce,
        }
    }
}
