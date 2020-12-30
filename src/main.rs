pub mod gfx;
pub mod util;
pub mod world;

use gfx::window::main_loop;
use std::env;
use util::Size2i;
use world::Params;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut params = Params::default();
    if args.len() > 1 {
        params.seed = Some(args[1].parse::<u64>().unwrap());
    } else {
        params.seed = None;
    }
    params.plant_grid_size = Size2i::new(200, 200);
    params.tick_interval = 0.04;
    params.agent_count = 5;
    main_loop(&params);
}
