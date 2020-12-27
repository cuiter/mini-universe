use sdl2::{
    render::Canvas,
    video::Window,
    pixels::Color,
    rect::Rect
};
use crate::world::World;
use crate::gfx::view::View;
use crate::util::{Vec2f, Vec2i, Rect2i, Rect2f, vec2f_to_vec2i, rect2f_to_rect2i, rect2i_collides};

const PLANT_COLOR: Color = Color::RGBA(96, 255, 32, 255);
const AGENT_COLOR: Color = Color::RGBA(20, 20, 200, 255);
const BACKGROUND_COLOR: Color = Color::RGBA(0, 0, 0, 255);

fn world_to_window_pos(view: &View, pos: Vec2f) -> Vec2f {
    let centered_pos = (pos - view.pos) * view.zoom;

    // The world is Y-up but the window is Y-down, so flip it.
    Vec2f::new(view.window_size.w as f32 / 2.0 + centered_pos.x,
               view.window_size.h as f32 / 2.0 - centered_pos.y)
}

fn world_to_window_rect(view: &View, rect: Rect2f) -> Rect2f {
    let bottom_left = world_to_window_pos(view, Vec2f::new(rect.x, rect.y));
    let top_right = world_to_window_pos(view, Vec2f::new(rect.x + rect.w, rect.y + rect.h));

    Rect2f::new(bottom_left.x, top_right.y, top_right.x - bottom_left.x, bottom_left.y - top_right.y)
}

pub fn draw_world(canvas: &mut Canvas<Window>, view: &View, world: &World) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let mut prev_color = Color::RGBA(0, 0, 0, 0);
    let window_rect = Rect2i::new(0, 0, view.window_size.w, view.window_size.h);

    // Draw plant grid.
    for row in 0..world.plant_grid.size.h {
        for col in 0..world.plant_grid.size.w {
            let draw_rect = rect2f_to_rect2i(world_to_window_rect(view, Rect2f::new(col as f32, row as f32, 1.0, 1.0)));
            if !rect2i_collides(draw_rect, window_rect) {
                // Plant cell is not on screen.
                continue;
            }

            let density = world.plant_grid.get_density(Vec2i::new(col as i32, row as i32));
            if density == 0 {
                continue;
            }
            let color = Color::RGBA(((PLANT_COLOR.r as i32 * density as i32) / 255) as u8,
                                    ((PLANT_COLOR.g as i32 * density as i32) / 255) as u8,
                                    ((PLANT_COLOR.b as i32 * density as i32) / 255) as u8,
                                    ((PLANT_COLOR.a as i32 * density as i32) / 255) as u8);
            if color != prev_color {
                canvas.set_draw_color(color);
                prev_color = color;
            }

            canvas.fill_rect(Rect::new(draw_rect.x, draw_rect.y, draw_rect.w, draw_rect.h)).unwrap();
        }
    }

    // Draw agents.
    canvas.set_draw_color(AGENT_COLOR);
    for agent in world.agents.iter() {
        let draw_rect = rect2f_to_rect2i(world_to_window_rect(view, agent.get_bounding_rect()));
        if !rect2i_collides(draw_rect, window_rect) {
            // Agent is not on screen.
            continue;
        }
        // TODO: draw circle instead of rect
        canvas.fill_rect(Rect::new(draw_rect.x, draw_rect.y, draw_rect.w, draw_rect.h)).unwrap();
    }
}
