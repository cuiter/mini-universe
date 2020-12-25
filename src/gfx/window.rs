use sdl2::event::{Event,WindowEvent};
use super::view::View;

const ENABLE_VSYNC: bool = true;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub fn main_loop() {
    let mut view = View::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Universe Simulation", WINDOW_WIDTH, WINDOW_HEIGHT)
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
                            view.window_width = width as u32;
                            view.window_height = height as u32;
                        },
                        _ => { },
                    }
                }

                _ => {},
            }
        }

        let cur_nano_time = time::precise_time_ns();
        let raw_d_time: f32 = (cur_nano_time - prev_nano_time) as f32 / 1e9f32 ;
        let d_time: f32 = if raw_d_time == 0f32 { 1e-9f32 } else { raw_d_time }; // to prevent divide by zero

        // world.update(d_time);
        prev_nano_time = cur_nano_time;

        // world.draw(&mut canvas);
        canvas.present();
    }
}
