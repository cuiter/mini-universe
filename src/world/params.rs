use crate::util::{Size2i};

pub struct Params {
    pub seed: Option<u64>,
    pub plant_grid_size: Size2i,
    pub tick_interval: f32,
}

impl Params {
    pub fn default() -> Params {
        Params {
            seed: None,
            plant_grid_size: Size2i::new(100, 100),
            tick_interval: 0.01
        }
    }
}
