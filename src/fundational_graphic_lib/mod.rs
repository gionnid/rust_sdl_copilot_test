extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::constant::{BACKGROUND_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn get_sdl_context_and_video_subsystem() -> (sdl2::Sdl, sdl2::VideoSubsystem) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    (sdl_context, video_subsystem)
}

pub fn run() {
    let (sdl_context, video_subsystem) = get_sdl_context_and_video_subsystem();

    let window = video_subsystem
        .window("SDL2 Window", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        draw_canvas(&mut canvas);
        ::std::thread::sleep(Duration::from_millis(100));
    }
}

fn draw_canvas(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    canvas.present();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_compiles() {
        run();
    }
}
