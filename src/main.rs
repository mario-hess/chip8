use std::time::Instant;
use std::env;

mod cpu;
mod instruction;
mod machine;
mod ppu;
mod program_counter;
mod ram;
mod registers;
mod rom;

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

    let mut start_time = Instant::now();

    loop {
        if start_time.elapsed().as_millis() > 2 {
            machine.run();
            start_time = Instant::now();
        }
    }
}
