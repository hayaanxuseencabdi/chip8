use crate::emulator::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::{thread, time::Duration};

mod emulator;

const PIXEL_WIDTH: usize = 25;
const PIXEL_HEIGHT: usize = 25;

const WINDOW_WIDTH: i32 = (PIXEL_WIDTH * DISPLAY_WIDTH) as i32;
const WINDOW_HEIGHT: i32 = (PIXEL_HEIGHT * DISPLAY_HEIGHT) as i32;

const FRAMES_PER_SECOND: u64 = 60;
const MICROSECONDS_PER_FRAME: u64 = 1_000_000 / FRAMES_PER_SECOND;

fn main() {
    // Set up the SDL2 window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    println!("Window dimensions: {}, {}", WINDOW_WIDTH, WINDOW_HEIGHT);

    let window = video_subsystem
        .window("CHIP-8", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::AppDidEnterBackground { .. } => { /* pause */ }
                Event::AppDidEnterForeground { .. } => { /* resume */ }
                Event::KeyDown {
                    repeat, keycode, ..
                } => {
                    if repeat {
                        continue;
                    }

                    let key = keycode.expect("No key in keycode on KeyDown event");
                    eprintln!("Key pressed: {}", key);
                    match key {
                        Keycode::Right => {
                            x = if x < WINDOW_WIDTH - PIXEL_WIDTH as i32 {
                                x + PIXEL_WIDTH as i32
                            } else {
                                0
                            };
                        }
                        Keycode::Left => {
                            x = if x > 0 {
                                x - PIXEL_WIDTH as i32
                            } else {
                                WINDOW_WIDTH - PIXEL_WIDTH as i32
                            };
                        }
                        Keycode::Down => {
                            y = if y < WINDOW_HEIGHT - PIXEL_HEIGHT as i32 {
                                y + PIXEL_HEIGHT as i32
                            } else {
                                0
                            };
                        }
                        Keycode::Up => {
                            y = if y > 0 {
                                y - PIXEL_HEIGHT as i32
                            } else {
                                WINDOW_HEIGHT - PIXEL_HEIGHT as i32
                            };
                        }
                        _ => {}
                    };
                }
                Event::KeyUp { .. } => { /* unpress key */ }
                _ => {}
            }
        }

        // Game loop
        let rectangle = Rect::new(x, y, PIXEL_WIDTH as u32, PIXEL_HEIGHT as u32);
        canvas.set_draw_color(Color::MAGENTA);
        canvas
            .fill_rect(rectangle)
            .expect("Failed to draw the rectangles");

        canvas.present();
        // 60 FPS
        thread::sleep(Duration::from_micros(MICROSECONDS_PER_FRAME));
    }
}
