use std::env;

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

    machine.run();
}
