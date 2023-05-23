use std::env;
use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod cpu;
mod instruction;
mod keyboard;
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
        .window("Chip8 Emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(64, 32).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        machine.keyboard.set_key(&mut event_pump);

        if machine.keyboard.key == Some(Keycode::Escape) {
            break 'running;
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

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
