use crate::gfx::assets::Assets;
use crate::gfx::view::View;
use crate::gfx::world::draw_world;
use crate::util::{time_ns, Size2i, Vec2f};
use crate::world::{Params, TimeController, World};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use vek::ops::Clamp;

const ENABLE_VSYNC: bool = true;
const WINDOW_SIZE: Size2i = Size2i::new(800, 600);

/// The main (GUI) loop of the program.
/// Creates an SDL2 window and runs an event loop.
pub fn main_loop(params: &Params) {
    let mut world = World::new(params);
    let mut time_controller = TimeController::new();
    let mut view = View::new(
        WINDOW_SIZE,
        Vec2f::new(
            params.plant_grid_size.w as f32 / 2.0,
            params.plant_grid_size.h as f32 / 2.0,
        ),
    );

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Mini Universe", WINDOW_SIZE.w, WINDOW_SIZE.h)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas;
    {
        let mut canvasbuilder = window.into_canvas();
        if ENABLE_VSYNC {
            canvasbuilder = canvasbuilder.present_vsync();
        }
        canvas = canvasbuilder.build().unwrap();
    }
    let mut texture_creator = canvas.texture_creator();
    let mut assets = Assets::load(&mut texture_creator);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut prev_nano_time = time_ns();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'event_loop;
                }

                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::SizeChanged(width, height) => {
                        view.window_size = Size2i::new(width as u32, height as u32);
                    }
                    _ => {}
                },

                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    if scancode == Scancode::R {
                        world = World::new(&params);
                    } else if scancode == Scancode::T {
                        time_controller.goto_prompt(params, &mut world);
                    } else {
                        view.key_down(scancode);
                    }
                }

                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    view.key_up(scancode);
                }

                Event::MouseWheel { y, .. } => {
                    view.change_zoom(y as f32);
                }

                _ => {}
            }
        }

        let cur_nano_time = time_ns();
        let raw_d_time: f32 = (cur_nano_time - prev_nano_time) as f32 / 1e9f32;
        // Clamp d_time between 1 nanosecond and 1 second to prevent divide by zero and runaway.
        let d_time: f32 = raw_d_time.clamped(1e-9f32, 1.0);

        view.tick(d_time);

        if !view.paused {
            time_controller.tick(params, &mut world, d_time * view.time_factor);
        }

        draw_world(&mut canvas, &mut assets, &view, &world);
        canvas.present();

        prev_nano_time = cur_nano_time;
    }
}
