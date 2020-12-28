use crate::util::{Vec2f, Rect2f, vec2f_to_vec2i, calculate_vec2f};
use crate::world::plant_grid::PlantGrid;

const DEFAULT_RADIUS: f32 = 2.0;
const MAX_SPEED: f32 = 8.0;
const MEASURE_DISTANCE: f32 = 4.0;
const MEASURE_ANGLE: f32 = 0.35;
const MOUTH_DISTANCE: f32 = 2.0;

pub struct Agent {
    pub radius: f32,
    pub pos: Vec2f,
    pub angle: f32, // radians
    pub energy: f32
}

struct Sensors {
    pub percepts: [f32; 2] // 0: left eye, 1: right eye
}

struct Actuators {
    pub commands: [f32; 2] // 0: left speed, 1: right speed
}

impl Agent {
    pub fn new(pos: Vec2f, angle: f32) -> Agent {
        Agent {
            radius: DEFAULT_RADIUS,
            pos,
            angle,
            energy: 1.0
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
            percepts: [left_density as f32 / 255.0, right_density as f32 / 255.0]
        }
    }

    fn calculate_actuators(&self, sensors: &Sensors) -> Actuators {
        Actuators {
            commands: [1.0 - sensors.percepts[0] * 0.99, 1.0 - sensors.percepts[1] * 0.99]
        }
    }

    fn apply_actuators(&mut self, actuators: &Actuators, plant_grid: &PlantGrid, d_time: f32) {
        let left_speed = actuators.commands[0] * (MAX_SPEED / 2.0);
        let right_speed = actuators.commands[1] * (MAX_SPEED / 2.0);

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

    pub fn tick(&mut self, plant_grid: &PlantGrid, d_time: f32) -> bool {
        let sensors = self.measure_sensors(plant_grid);
        let actuators = self.calculate_actuators(&sensors);
        self.apply_actuators(&actuators, plant_grid, d_time);

        let density_at_mouth = plant_grid.get_density(vec2f_to_vec2i(self.get_mouth_pos()));

        density_at_mouth > 0
    }
}
