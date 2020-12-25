#[derive(Clone)]
pub struct View {
    pub window_width: u32,
    pub window_height: u32,
    pub zoom_level: f32,
    pub paused: bool,
}

impl View {
    pub fn new(window_width: u32, window_height: u32) -> View {
        View {
            window_width,
            window_height,
            zoom_level: 1.0,
            paused: false,
        }
    }
}
