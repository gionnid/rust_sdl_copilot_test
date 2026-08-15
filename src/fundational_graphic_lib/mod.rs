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
        // 1. Input: process all pending SDL events. Add new key/mouse handling in handle_events().
        if !handle_events(&mut event_pump) {
            break 'running;
        }

        // 2. Logic: advance game/simulation state. Add new state updates in update().
        update();

        // 3. Output: draw the current state. Add new drawing calls in render().
        render(&mut canvas);

        ::std::thread::sleep(Duration::from_millis(100));
    }
}

/// Processes all pending SDL events. Returns `false` when the app should quit.
fn handle_events(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {}
        }
    }
    true
}

/// Updates game/simulation state for the current frame.
fn update() {
    // Add per-frame logic (movement, physics, timers, etc.) here.
}

/// Draws the current frame to the canvas.
fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    // Add drawing calls here.

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
