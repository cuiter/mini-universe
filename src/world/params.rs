use crate::util::Size2i;

/// World parameters that might be modified by an end user.
pub struct Params {
    pub seed: Option<u64>,
    pub plant_grid_size: Size2i,
    pub tick_interval: f32,
    pub agent_count: u32,
    pub evolution: bool,
}

impl Params {
    /// Creates a Params with the default values.
    pub fn default() -> Params {
        Params {
            seed: None,
            plant_grid_size: Size2i::new(200, 200),
            tick_interval: 0.04,
            agent_count: 5,
            evolution: true,
        }
    }
}
