use std::time::Instant;

use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::ram::Ram;
use crate::rom::Rom;
use crate::timer::Timer;

pub const ROM_START_ADDRESS: u16 = 0x200;

pub struct Machine {
    cpu: Cpu,
    ram: Ram,
    pub ppu: Ppu,
    timer: Timer,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            ppu: Ppu::new(),
            timer: Timer::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Rom) {
        for (index, byte) in rom.data.iter().enumerate() {
            self.ram.write_byte(index as u16 + ROM_START_ADDRESS, *byte);
        }
    }

    pub fn run(&mut self) {
        self.cpu
            .execute_instruction(&mut self.ram, &mut self.ppu, &mut self.timer);
    }

    pub fn draw_pixels(&mut self) {
        for h in 0..32 {
            for w in 0..64 {
                if self.ppu.display[h as usize][w as usize] == 0 {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}
