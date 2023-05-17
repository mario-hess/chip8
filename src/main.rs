use std::time::{Instant};

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
    let rom = Rom::new("roms/INVADERS");
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
