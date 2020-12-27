use crate::util::{Size2i, Vec2f};

const DEFAULT_ZOOM: f32 = 20.0;
const ZOOM_CHANGE_FACTOR: f32 = 1.2;

#[derive(Clone)]
pub struct View {
    pub window_size: Size2i,
    pub pos: Vec2f,
    pub zoom: f32,
    pub paused: bool,
}

impl View {
    pub fn new(window_size: Size2i, pos: Vec2f) -> View {
        View {
            window_size,
            pos,
            zoom: DEFAULT_ZOOM, // Pixels per plant cell
            paused: false,
        }
    }

    /// Changes the zoom level.
    /// If scroll is positive, zooms in.
    /// If scroll is negative, zooms out.
    /// For reference, scroll = 1.0 should mean one mouse wheel up tick;
    pub fn change_zoom(&mut self, scroll: f32) {
        self.zoom *= ZOOM_CHANGE_FACTOR.powf(scroll);
    }
}
