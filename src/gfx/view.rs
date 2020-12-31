use crate::util::{Size2i, Vec2f};
use sdl2::keyboard::Scancode;
use std::collections::HashMap;

const POS_MOVE_FACTOR: f32 = 500.0; // Pixels per second
const DEFAULT_ZOOM: f32 = 20.0;
const ZOOM_CHANGE_FACTOR: f32 = 1.2; // Exponential
const TIME_FACTOR_CHANGE_FACTOR: f32 = 2.0; // Exponential

#[derive(Clone)]
/// A projection into the world.
/// Also keeps track of the paused state, time factor, window size and pressed keys.
pub struct View {
    pub pos: Vec2f,
    pub zoom: f32,
    pub time_factor: f32,
    pub paused: bool,
    pub window_size: Size2i,
    keys: HashMap<Scancode, bool>,
}

impl View {
    /// Creates a new view with the specified window size and position.
    pub fn new(window_size: Size2i, pos: Vec2f) -> View {
        View {
            pos,
            zoom: DEFAULT_ZOOM, // Pixels per plant cell
            paused: false,
            time_factor: 1.0,
            window_size,
            keys: HashMap::new(),
        }
    }

    /// Changes the zoom level.
    /// If scroll is positive, zooms in.
    /// If scroll is negative, zooms out.
    /// For reference, scroll = 1.0 should mean one mouse wheel up tick;
    pub fn change_zoom(&mut self, scroll: f32) {
        self.zoom *= ZOOM_CHANGE_FACTOR.powf(scroll);
    }

    /// Handles a "key down" event and modifies the view if necessary.
    pub fn key_down(&mut self, key: Scancode) {
        self.keys.insert(key, true);

        // Adjust time parameters based on keyboard input.
        match key {
            Scancode::Space => {
                self.paused = !self.paused;
            }
            Scancode::Comma => {
                self.time_factor /= TIME_FACTOR_CHANGE_FACTOR;
                println!("time factor: {}", self.time_factor);
            }
            Scancode::Period => {
                self.time_factor *= TIME_FACTOR_CHANGE_FACTOR;
                println!("time factor: {}", self.time_factor);
            }
            _ => {}
        }
    }

    /// Handles a "key up" event.
    pub fn key_up(&mut self, key: Scancode) {
        self.keys.insert(key, false);
    }

    /// Returns whether a key is pressed.
    fn get_key(&self, key: Scancode) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    /// Updates the view position.
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
