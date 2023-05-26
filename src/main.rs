use std::env;
use std::time::Duration;

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

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const FPS_RATE: u32 = 1_000_000_000u32 / 60;

fn main() {
    let mut shift_quirk = false;
    let mut jump_quirk = false;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: No file path provided.");
    }
    if args.len() > 2 && &args[2] == "shift_quirk" {
        shift_quirk = true;
    }
    if args.len() > 3 && &args[3] == "jump_quirk" {
        jump_quirk = true;
    }

    let file_path = "roms/".to_owned() + &args[1];

    let rom = Rom::build(&file_path).expect("Error reading file.");
    let mut machine = Machine::new(shift_quirk, jump_quirk);
    machine.load_rom(rom);

    let sdl_context = sdl2::init().expect("Error initializing SDL.");
    let video_subsystem = sdl_context
        .video()
        .expect("Error initializing VideoSubSystem");

    let window = video_subsystem
        .window(
            "Chip8 Emulator",
            (ppu::SCREEN_WIDTH * 10) as u32,
            (ppu::SCREEN_HEIGHT * 10) as u32,
        )
        .position_centered()
        .build()
        .expect("Error building window.");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Error building canvas.");
    canvas
        .set_logical_size(ppu::SCREEN_WIDTH as u32, ppu::SCREEN_HEIGHT as u32)
        .expect("Error setting logical size.");

    let mut event_pump = sdl_context.event_pump().unwrap();

    while !machine.keyboard.escape_pressed {
        machine.keyboard.set_key(&mut event_pump);

        canvas.set_draw_color(BLACK);
        canvas.clear();

        // Chip8 runs roughly 10 instructions per frame
        for _ in 0..10 {
            machine.run_instruction();
        }

        canvas.set_draw_color(WHITE);

        for height in 0..ppu::SCREEN_HEIGHT {
            for width in 0..ppu::SCREEN_WIDTH {
                if machine.ppu.display[height][width] == 1 {
                    canvas
                        .draw_point(Point::new(width as i32, height as i32))
                        .expect("Error drawing pixel.");
                }
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, FPS_RATE));
    }
}
