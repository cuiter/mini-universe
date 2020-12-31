use rand_pcg;
use std::time::{SystemTime, UNIX_EPOCH};
use vek;

pub type Vec2f = vek::Vec2<f32>;
pub type Vec2i = vek::Vec2<i32>;
pub type Vec3f = vek::Vec3<f32>;
pub type Size2f = vek::Extent2<f32>;
pub type Size2i = vek::Extent2<u32>;
pub type Rect2f = vek::Rect<f32, f32>;
pub type Rect2i = vek::Rect<i32, u32>;
/// Random number generator used throughout the program.
pub type WRng = rand_pcg::Pcg32;

#[inline]
/// Convert a Vec2f to a Vec2i, removing the non-integer parts.
pub fn vec2f_to_vec2i(pos: Vec2f) -> Vec2i {
    Vec2i::new(pos.x as i32, pos.y as i32)
}

#[inline]
/// Convert a Rect2f to a Rect2i, removing the non-integer parts.
pub fn rect2f_to_rect2i(rect: Rect2f) -> Rect2i {
    Rect2i::new(
        rect.x as i32,
        rect.y as i32,
        rect.w.ceil() as u32,
        rect.h.ceil() as u32,
    )
}

#[inline]
/// Check whether two Rect2i rectangles collide.
pub fn rect2i_collides(rect_1: Rect2i, rect_2: Rect2i) -> bool {
    vek::Rect::new(rect_1.x, rect_1.y, rect_1.w as i32, rect_1.h as i32).collides_with_rect(
        vek::Rect::new(rect_2.x, rect_2.y, rect_2.w as i32, rect_2.h as i32),
    )
}

/// Calculate the 2D vector from a specified speed and angle.
pub fn calculate_vec2f(speed: f32, angle: f32) -> Vec2f {
    Vec2f::new(angle.cos() * speed, angle.sin() * speed)
}

/// Get the current Unix time in nanoseconds.
pub fn time_ns() -> u128 {
    let time_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    time_since_epoch.as_nanos()
}
