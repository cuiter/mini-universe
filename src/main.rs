pub mod gfx;
pub mod util;
pub mod world;

use gfx::window::main_loop;
use std::env;
use world::Params;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut params = Params::default();
    if args.len() > 1 {
        params.seed = Some(args[1].parse::<u64>().unwrap());
    } else {
        params.seed = None;
    }

    main_loop(&params);
}
