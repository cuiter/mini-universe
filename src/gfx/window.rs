use sdl2::event::{Event,WindowEvent};
use crate::util::{Size2i, Vec2f};
use crate::gfx::view::View;
use crate::world::{World};
use crate::gfx::world::draw_world;

const ENABLE_VSYNC: bool = true;
const WINDOW_SIZE: Size2i = Size2i::new(800, 600);
const PLANT_GRID_SIZE: Size2i = Size2i::new(50, 50);

pub fn main_loop() {
    let mut world = World::new(PLANT_GRID_SIZE);
    let mut view = View::new(WINDOW_SIZE, Vec2f::new(PLANT_GRID_SIZE.w as f32 / 2.0, PLANT_GRID_SIZE.h as f32 / 2.0));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Universe Simulation", WINDOW_SIZE.w, WINDOW_SIZE.h)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut canvas;
    {
        let mut canvasbuilder = window.into_canvas();
        if ENABLE_VSYNC {
            canvasbuilder = canvasbuilder.present_vsync();
        }
        canvas = canvasbuilder.build().unwrap();
    }
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut prev_nano_time = time::precise_time_ns();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'event_loop; },

                Event::Window {win_event, ..} => {
                    match win_event {
                        WindowEvent::SizeChanged(width, height) => {
                            view.window_size = Size2i::new(width as u32, height as u32);
                        },
                        _ => { },
                    }
                }

                Event::MouseWheel {y, ..} => {
                    view.change_zoom(y as f32);
                }

                _ => {},
            }
        }

        let cur_nano_time = time::precise_time_ns();
        let raw_d_time: f32 = (cur_nano_time - prev_nano_time) as f32 / 1e9f32 ;
        let d_time: f32 = if raw_d_time == 0f32 { 1e-9f32 } else { raw_d_time }; // to prevent divide by zero

        world.tick(d_time);
        prev_nano_time = cur_nano_time;

        draw_world(&mut canvas, &view, &world);
        canvas.present();
    }
}
