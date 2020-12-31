use crate::world::{Params, World};

/// A struct that handles calling the world.tick() method
/// the right amount of times. Also provides time travel.
pub struct TimeController {
    time_left: f32,
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController { time_left: 0.0 }
    }

    /// Run the world for the specified amount of time.
    pub fn tick(&mut self, params: &Params, world: &mut World, d_time: f32) {
        self.time_left += d_time;
        while self.time_left > params.tick_interval {
            world.tick(params, params.tick_interval);
            self.time_left -= params.tick_interval;
        }
    }

    /// Travel to the specified time.
    pub fn goto(&mut self, params: &Params, world: &mut World, new_time: f64) {
        if new_time < world.time {
            // If the time is in the past,
            // create a new world and run the simulation again.
            *world = World::new(params);
        }
        while world.time < new_time {
            world.tick(params, params.tick_interval);
        }
    }

    /// Prompt the user on the command-line for time travel.
    pub fn goto_prompt(&mut self, params: &Params, world: &mut World) {
        println!("enter new time:");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim_end().to_string();
        match line.parse::<f64>() {
            Ok(new_time) => {
                self.goto(params, world, new_time);
            }
            Err(..) => {
                println!("{} is not a valid time", line);
            }
        }
    }
}
