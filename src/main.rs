use crate::emulator::{Emulator, DISPLAY_HEIGHT, DISPLAY_WIDTH};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::{fs::File, io::Read, thread, time::Duration};

mod emulator;

const PIXEL_WIDTH: usize = 25;
const PIXEL_HEIGHT: usize = 25;

const WINDOW_WIDTH: i32 = (PIXEL_WIDTH * DISPLAY_WIDTH) as i32;
const WINDOW_HEIGHT: i32 = (PIXEL_HEIGHT * DISPLAY_HEIGHT) as i32;

const FRAMES_PER_SECOND: u64 = 60;
const MICROSECONDS_PER_FRAME: u64 = 1_000_000 / FRAMES_PER_SECOND;

fn main() {
    let mut emulator = Emulator::new();
    let mut file = File::open("tests/c8_test.c8").expect("Unable to open the ROM");
    let mut rom = [0u8; 0xE00];
    file.read(&mut rom).unwrap();
    emulator.load(&rom);

    // Set up the SDL2 window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    eprintln!("Window dimensions: {}, {}", WINDOW_WIDTH, WINDOW_HEIGHT);

    let window = video_subsystem
        .window("CHIP-8", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
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
                        Keycode::Num1 => {
                            emulator.key_press(0x1);
                        }
                        Keycode::Num2 => {
                            emulator.key_press(0x2);
                        }
                        Keycode::Num3 => {
                            emulator.key_press(0x3);
                        }
                        Keycode::Num4 => {
                            emulator.key_press(0xC);
                        }

                        Keycode::Q => {
                            emulator.key_press(0x4);
                        }
                        Keycode::W => {
                            emulator.key_press(0x5);
                        }
                        Keycode::E => {
                            emulator.key_press(0x6);
                        }
                        Keycode::R => {
                            emulator.key_press(0xD);
                        }

                        Keycode::A => {
                            emulator.key_press(0x7);
                        }
                        Keycode::S => {
                            emulator.key_press(0x8);
                        }
                        Keycode::D => {
                            emulator.key_press(0x9);
                        }
                        Keycode::F => {
                            emulator.key_press(0xE);
                        }

                        Keycode::Z => {
                            emulator.key_press(0xA);
                        }
                        Keycode::X => {
                            emulator.key_press(0x0);
                        }
                        Keycode::C => {
                            emulator.key_press(0xB);
                        }
                        Keycode::V => {
                            emulator.key_press(0xF);
                        }

                        _ => {}
                    };
                }
                Event::KeyUp {
                    repeat, keycode, ..
                } => {
                    if repeat {
                        continue;
                    }

                    let key = keycode.expect("No key in keycode on KeyUp event");
                    eprintln!("Key released: {}", key);

                    match key {
                        Keycode::Num1 => {
                            emulator.key_release(0x1);
                        }
                        Keycode::Num2 => {
                            emulator.key_release(0x2);
                        }
                        Keycode::Num3 => {
                            emulator.key_release(0x3);
                        }
                        Keycode::Num4 => {
                            emulator.key_release(0xC);
                        }

                        Keycode::Q => {
                            emulator.key_release(0x4);
                        }
                        Keycode::W => {
                            emulator.key_release(0x5);
                        }
                        Keycode::E => {
                            emulator.key_release(0x6);
                        }
                        Keycode::R => {
                            emulator.key_release(0xD);
                        }

                        Keycode::A => {
                            emulator.key_release(0x7);
                        }
                        Keycode::S => {
                            emulator.key_release(0x8);
                        }
                        Keycode::D => {
                            emulator.key_release(0x9);
                        }
                        Keycode::F => {
                            emulator.key_release(0xE);
                        }

                        Keycode::Z => {
                            emulator.key_release(0xA);
                        }
                        Keycode::X => {
                            emulator.key_release(0x0);
                        }
                        Keycode::C => {
                            emulator.key_release(0xB);
                        }
                        Keycode::V => {
                            emulator.key_release(0xF);
                        }

                        _ => {}
                    }
                }
                _ => {}
            }
        }

        emulator.instruction_cycle();
        canvas.set_draw_color(Color::WHITE);

        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                if emulator.pixel_at(x, y) {
                    let rectangle = Rect::new(
                        (x * PIXEL_WIDTH) as i32,
                        (y * PIXEL_HEIGHT) as i32,
                        PIXEL_WIDTH as u32,
                        PIXEL_HEIGHT as u32,
                    );
                    canvas
                        .fill_rect(rectangle)
                        .expect("Failed to draw the rectangles");
                }
            }
        }

        canvas.present();
        // 60 FPS
        thread::sleep(Duration::from_micros(MICROSECONDS_PER_FRAME));
    }
}
