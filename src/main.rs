pub mod gfx;
pub mod world;
pub mod util;

use gfx::window::main_loop;
use world::Params;
use util::{Size2i};

fn main() {
    let mut params = Params::default();
    params.seed = None;
    params.plant_grid_size = Size2i::new(200, 200);
    params.tick_interval = 0.04;
    main_loop(&params);
}
