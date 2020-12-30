use crate::world::{World, Params};

pub struct TimeController {
    time_left: f32
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController { time_left: 0.0 }
    }

    pub fn tick(&mut self, params: &Params, world: &mut World, d_time: f32) {
        self.time_left += d_time;
        while self.time_left > params.tick_interval {
            world.tick(params, params.tick_interval);
            self.time_left -= params.tick_interval;
        }
    }

    pub fn goto(&mut self, params: &Params, world: &mut World, new_time: f32) {
        if new_time < world.time {
            *world = World::new(params);
        }
        while world.time < new_time {
            world.tick(params, params.tick_interval);
        }
    }
}
