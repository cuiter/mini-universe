use crate::gfx::assets::Assets;
use crate::gfx::view::View;
use crate::util::{rect2i_collides, vec2f_to_vec2i, Rect2f, Rect2i, Vec2f, Vec2i};
use crate::world::World;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

const BACKGROUND_COLOR: Color = Color::RGBA(0, 0, 0, 255);
const PLANT_COLOR: Color = Color::RGBA(96, 255, 32, 255);
const AGENT_MEASURE_COLOR: Color = Color::RGBA(100, 100, 255, 200);
const AGENT_MEASURE_SIZE: f32 = 0.25;

fn world_to_window_pos(view: &View, pos: Vec2f) -> Vec2f {
    let centered_pos = (pos - view.pos) * view.zoom;

    // The world is Y-up but the window is Y-down, so flip it.
    Vec2f::new(
        view.window_size.w as f32 / 2.0 + centered_pos.x,
        view.window_size.h as f32 / 2.0 - centered_pos.y,
    )
}

fn world_to_window_rect(view: &View, rect: Rect2f) -> Rect2i {
    let bottom_left = vec2f_to_vec2i(world_to_window_pos(view, Vec2f::new(rect.x, rect.y)));
    let top_right = vec2f_to_vec2i(world_to_window_pos(
        view,
        Vec2f::new(rect.x + rect.w, rect.y + rect.h),
    ));

    Rect2i::new(
        bottom_left.x,
        top_right.y,
        (top_right.x - bottom_left.x) as u32,
        (bottom_left.y - top_right.y) as u32,
    )
}

pub fn draw_world(canvas: &mut Canvas<Window>, assets: &mut Assets, view: &View, world: &World) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let mut prev_color = Color::RGBA(0, 0, 0, 0);
    let window_rect = Rect2i::new(0, 0, view.window_size.w, view.window_size.h);

    // Draw plant grid.
    for row in 0..world.plant_grid.size.h {
        for col in 0..world.plant_grid.size.w {
            let draw_rect =
                world_to_window_rect(view, Rect2f::new(col as f32, row as f32, 1.0, 1.0));
            if !rect2i_collides(draw_rect, window_rect) {
                // Plant cell is not on screen.
                continue;
            }

            let density = world
                .plant_grid
                .get_density_unchecked(Vec2i::new(col as i32, row as i32));
            if density == 0 {
                continue;
            }
            let color = Color::RGBA(
                ((PLANT_COLOR.r as i32 * density as i32) / 255) as u8,
                ((PLANT_COLOR.g as i32 * density as i32) / 255) as u8,
                ((PLANT_COLOR.b as i32 * density as i32) / 255) as u8,
                ((PLANT_COLOR.a as i32 * density as i32) / 255) as u8,
            );
            if color != prev_color {
                canvas.set_draw_color(color);
                prev_color = color;
            }

            canvas
                .fill_rect(Rect::new(
                    draw_rect.x,
                    draw_rect.y,
                    draw_rect.w,
                    draw_rect.h,
                ))
                .unwrap();
        }
    }

    // Draw agents.
    canvas.set_draw_color(AGENT_MEASURE_COLOR);
    for agent in world.agents.iter() {
        let agent_color = agent.genes.get_color();
        assets.agent_sprite.set_color_mod(
            (agent_color.x * 255.0) as u8,
            (agent_color.y * 255.0) as u8,
            (agent_color.z * 255.0) as u8,
        );
        let draw_rect = world_to_window_rect(view, agent.get_bounding_rect());
        if !rect2i_collides(draw_rect, window_rect) {
            // Agent is not on screen.
            continue;
        }

        // Draw agent
        canvas
            .copy_ex(
                &assets.agent_sprite,
                None,
                Rect::new(draw_rect.x, draw_rect.y, draw_rect.w, draw_rect.h),
                -agent.angle.to_degrees() as f64,
                Point::new(draw_rect.w as i32 / 2, draw_rect.h as i32 / 2),
                false,
                false,
            )
            .unwrap();
        canvas
            .copy_ex(
                &assets.agent_eyes_sprite,
                None,
                Rect::new(draw_rect.x, draw_rect.y, draw_rect.w, draw_rect.h),
                -agent.angle.to_degrees() as f64,
                Point::new(draw_rect.w as i32 / 2, draw_rect.h as i32 / 2),
                false,
                false,
            )
            .unwrap();

        // Draw measurement points.
        let left_measure_pos = agent.get_left_measure_pos();
        let right_measure_pos = agent.get_right_measure_pos();
        let left_measure_rect = world_to_window_rect(
            view,
            Rect2f::new(
                left_measure_pos.x - AGENT_MEASURE_SIZE / 2.0,
                left_measure_pos.y - AGENT_MEASURE_SIZE / 2.0,
                AGENT_MEASURE_SIZE,
                AGENT_MEASURE_SIZE,
            ),
        );
        let right_measure_rect = world_to_window_rect(
            view,
            Rect2f::new(
                right_measure_pos.x - AGENT_MEASURE_SIZE / 2.0,
                right_measure_pos.y - AGENT_MEASURE_SIZE / 2.0,
                AGENT_MEASURE_SIZE,
                AGENT_MEASURE_SIZE,
            ),
        );
        canvas
            .fill_rect(Rect::new(
                left_measure_rect.x,
                left_measure_rect.y,
                left_measure_rect.w,
                left_measure_rect.h,
            ))
            .unwrap();
        canvas
            .fill_rect(Rect::new(
                right_measure_rect.x,
                right_measure_rect.y,
                right_measure_rect.w,
                right_measure_rect.h,
            ))
            .unwrap();
    }
}
