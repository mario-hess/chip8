use crate::cpu::Cpu;
use crate::keyboard::Keyboard;
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
    pub keyboard: Keyboard,
    shift_quirk: bool,
    jump_quirk: bool,
}

impl Machine {
    pub fn new(shift_quirk: bool, jump_quirk: bool) -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            ppu: Ppu::new(),
            timer: Timer::new(),
            keyboard: Keyboard::new(),
            shift_quirk,
            jump_quirk,
        }
    }

    pub fn load_rom(&mut self, rom: Rom) {
        for (index, byte) in rom.data.iter().enumerate() {
            self.ram.write_byte(index as u16 + ROM_START_ADDRESS, *byte);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.execute_instruction(
            &mut self.ram,
            &mut self.ppu,
            &mut self.timer,
            &mut self.keyboard,
            self.shift_quirk,
            self.jump_quirk,
        );
    }
}
