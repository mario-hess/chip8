use std::env;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

mod cpu;
mod instruction;
mod machine;
mod ppu;
mod program_counter;
mod ram;
mod registers;
mod rom;
mod timer;

use machine::Machine;
use rom::Rom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: No file path provided.");
    }
    let file_path = "roms/".to_owned() + &args[1];

    let rom = Rom::build(&file_path).expect("Error reading file.");
    let mut machine = Machine::new();
    machine.load_rom(rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Handle events...
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
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for _ in 0..10 {
            machine.run();
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for height in 0..32 {
            for width in 0..64 {
                if machine.ppu.display[height][width] == 1 {
                    canvas
                        .draw_point(Point::new(width as i32, height as i32))
                        .unwrap();
                }
            }
        }
        canvas.set_logical_size(64, 32).unwrap();

        canvas.present();

        // machine.draw_pixels();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
