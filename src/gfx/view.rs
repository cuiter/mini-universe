use crate::util::{Size2i, Vec2f};
use std::collections::HashMap;
use sdl2::keyboard::Scancode;

const DEFAULT_ZOOM: f32 = 20.0;
const ZOOM_CHANGE_FACTOR: f32 = 1.2;
const POS_MOVE_FACTOR: f32 = 500.0; // Pixels per second

#[derive(Clone)]
pub struct View {
    pub window_size: Size2i,
    pub pos: Vec2f,
    pub zoom: f32,
    pub paused: bool,
    keys: HashMap<Scancode, bool>
}

impl View {
    pub fn new(window_size: Size2i, pos: Vec2f) -> View {
        View {
            window_size,
            pos,
            zoom: DEFAULT_ZOOM, // Pixels per plant cell
            paused: false,
            keys: HashMap::new()
        }
    }

    /// Changes the zoom level.
    /// If scroll is positive, zooms in.
    /// If scroll is negative, zooms out.
    /// For reference, scroll = 1.0 should mean one mouse wheel up tick;
    pub fn change_zoom(&mut self, scroll: f32) {
        self.zoom *= ZOOM_CHANGE_FACTOR.powf(scroll);
    }

    pub fn key_down(&mut self, key: Scancode) {
        self.keys.insert(key, true);
    }

    pub fn key_up(&mut self, key: Scancode) {
        self.keys.insert(key, false);
    }

    fn get_key(&self, key: Scancode) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn tick(&mut self, d_time: f32) {
        // Move view position based on keyboard input.
        let mut pos_diff = Vec2f::new(0.0, 0.0);
        if self.get_key(Scancode::W) || self.get_key(Scancode::Up) {
            pos_diff.y += 1.0;
        }
        if self.get_key(Scancode::A) || self.get_key(Scancode::Left) {
            pos_diff.x -= 1.0;
        }
        if self.get_key(Scancode::S) || self.get_key(Scancode::Down) {
            pos_diff.y -= 1.0;
        }
        if self.get_key(Scancode::D) || self.get_key(Scancode::Right) {
            pos_diff.x += 1.0;
        }
        self.pos += (pos_diff * POS_MOVE_FACTOR * d_time) / self.zoom;
    }
}
