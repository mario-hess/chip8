use std::time::Instant;

use crate::instruction::Instruction;
use crate::ppu::Ppu;
use crate::program_counter::ProgramCounter;
use crate::ram::Ram;
use crate::registers::Registers;

const MASK_MSB: u16 = 0xF000;
const MASK_ADDR: u16 = 0x0FFF;
const MASK_NN: u16 = 0x00FF;
const MASK_N: u16 = 0x000F;
const MASK_X: u16 = 0x0F00;
const MASK_Y: u16 = 0x00F0;

pub struct Cpu {
    pub registers: Registers,
    pub program_counter: ProgramCounter,
    pub delay_timer: u8,
    timer_updated: Instant,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            program_counter: ProgramCounter::new(),
            delay_timer: 0,
            timer_updated: Instant::now(),
        }
    }

    pub fn execute_instruction(&mut self, ram: &mut Ram, ppu: &mut Ppu) {
        // All instructions are 2 bytes long and are stored most-significant-byte first.
        let instruction = ram.get_instruction(self.program_counter.get_value());

        // msb  - the upper 4 bits of the instruction
        // addr - the lowest 12 bits of the instruction
        // nn   - the middle 8 bits of the instruction
        // n    - the lowest 4 bits of the instruction
        // x    - the lower 4 bits of the high byte of the instruction
        // y    - the upper 4 bits of the low byte of the instruction
        let (msb, addr, nn, n, x, y) = self.mask_opcodes(instruction);

        // debug
        println!("Instruction: {:#X}", instruction);
        println!(
            "msb: {:#X}, addr: {:#X}, nn: {:#X}, n: {:#X}, x: {:#X}, y: {:#X}",
            msb, addr, nn, n, x, y
        );

        match msb {
            0x0 => Instruction::exec_0x0(self, nn, ppu),
            0x1 => Instruction::exec_0x1(self, addr),
            0x2 => Instruction::exec_0x2(self, addr),
            0x3 => Instruction::exec_0x3(self, nn, x),
            0x4 => Instruction::exec_0x4(self, nn, x),
            0x6 => Instruction::exec_0x6(self, nn, x),
            0x7 => Instruction::exec_0x7(self, nn, x),
            0x8 => Instruction::exec_0x8(self, n, x, y),
            0xA => Instruction::exec_0xa(self, addr),
            0xD => Instruction::exec_0xd(self, n, x, y, ram, ppu),
            0xE => Instruction::exec_0xe(self, nn, x),
            0xF => Instruction::exec_0xf(self, nn, x, ram),
            _ => panic!("Invalid instruction."),
        }
    }

    fn mask_opcodes(&self, instruction: u16) -> (u8, u16, u8, u8, u8, u8) {
        (
            ((instruction & MASK_MSB) >> 12) as u8,
            (instruction & MASK_ADDR),
            (instruction & MASK_NN) as u8,
            (instruction & MASK_N) as u8,
            ((instruction & MASK_X) >> 8) as u8,
            ((instruction & MASK_Y) >> 4) as u8,
        )
    }

    pub fn draw_pixels(&self, ppu: &mut Ppu) {
        for h in 0..32 {
            for w in 0..64 {
                if ppu.display[h as usize][w as usize] == 0 {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.timer_updated = Instant::now();
        self.delay_timer = value;
    }
    pub fn get_delay_timer(&self) -> u8 {
        let ms_diff = (Instant::now() - self.timer_updated).as_millis();
        let ticks = ms_diff / 16;

        if ticks >= self.delay_timer as u128 {
            0
        } else {
            self.delay_timer - ticks as u8
        }
    }
}
