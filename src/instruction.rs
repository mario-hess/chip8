use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::ram::Ram;

const MASK_LSBIT: u8 = 0b0000_0001;

pub struct Instruction {}

impl Instruction {
    pub fn exec_0x0(cpu: &mut Cpu, nn: u8, ppu: &mut Ppu) {
        match nn {
            0xEE => {
                // 00EE
                // Returns from a subroutine
                let value = cpu.registers.stack_pop();
                cpu.program_counter.set_value(value);
            }
            0xE0 => {
                // 00E0
                // Clears the screen
                ppu.clear();
                cpu.program_counter.next();
            }
            _ => panic!("Invalid 0x00nn instruction"),
        }
    }

    pub fn exec_0x1(cpu: &mut Cpu, addr: u16) {
        // 1NNN
        // Jumps to address NNN
        cpu.program_counter.set_value(addr);
    }

    pub fn exec_0x2(cpu: &mut Cpu, addr: u16) {
        // 2NNN
        // Calls subroutine at NNN
        let value = cpu.program_counter.get_value() + 2;
        cpu.registers.stack_push(value);
        cpu.program_counter.set_value(addr);
    }

    pub fn exec_0x3(cpu: &mut Cpu, nn: u8, x: u8) {
        // 3XNN
        // Skips the next instruction if VX equals NN (usually the
        // next instruction is a jump to skip a code block).
        let vx = cpu.registers.get_vn(x);
        if vx == nn {
            cpu.program_counter.skip_next();
        } else {
            cpu.program_counter.next();
        }
    }

    pub fn exec_0x4(cpu: &mut Cpu, nn: u8, x: u8) {
        // 4XNN
        // Skips the next instruction if VX does not equal NN
        // (usually the next instruction is a jump to skip a code block)
        let vx = cpu.registers.get_vn(x);
        if vx != nn {
            cpu.program_counter.skip_next();
        } else {
            cpu.program_counter.next();
        }
    }

    pub fn exec_0x6(cpu: &mut Cpu, nn: u8, x: u8) {
        // 6XNN
        // Sets VX to NN
        cpu.registers.set_vn(x, nn);
        cpu.program_counter.next();
    }

    pub fn exec_0x7(cpu: &mut Cpu, nn: u8, x: u8) {
        // 7XNN
        // Adds NN to VX (carry flag is not changed)
        let vx = cpu.registers.get_vn(x);
        let result = vx.wrapping_add(nn);
        cpu.registers.set_vn(x, result);
        cpu.program_counter.next();
    }

    pub fn exec_0x8(cpu: &mut Cpu, n: u8, x: u8, y: u8) {
        match n {
            0x0 => {
                // 8XY0
                // Sets VX to the value of VY
                let vy = cpu.registers.get_vn(y);
                cpu.registers.set_vn(x, vy);
                cpu.program_counter.next();
            }
            0x2 => {
                // 8XY2
                // Sets VX to VX and VY. (bitwise AND operation)
                let vx = cpu.registers.get_vn(x);
                let vy = cpu.registers.get_vn(y);
                let result = vx & vy;
                cpu.registers.set_vn(x, result);
                cpu.program_counter.next();
            }
            0x3 => {
                // 8XY3
                // Sets VX to VX xor VY
                let vx = cpu.registers.get_vn(x);
                let vy = cpu.registers.get_vn(y);
                let result = vx ^ vy;
                cpu.registers.set_vn(x, result);
                cpu.program_counter.next();
            }
            0x4 => {
                // 8XY4
                // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not
                let vx = cpu.registers.get_vn(x);
                let vy = cpu.registers.get_vn(y);
                let result = vx as u16 + vy as u16;
                cpu.registers.set_vn(x, result as u8);
                if result > 0xFF {
                    cpu.registers.set_vn(0xF, 1);
                } else {
                    cpu.registers.set_vn(0xF, 0);
                }
                cpu.program_counter.next();
            }
            0x5 => {
                // 8XY5
                // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not
                let vx = cpu.registers.get_vn(x);
                let vy = cpu.registers.get_vn(y);
                let result = vx as i8 - vy as i8;
                cpu.registers.set_vn(x, result as u8);
                if result < 0 {
                    cpu.registers.set_vn(0xF, 0);
                } else {
                    cpu.registers.set_vn(0xF, 1);
                }
                cpu.program_counter.next();
            }
            0x6 => {
                // 8XY6
                // Store the value of register VY shifted right one bit in register VX
                // Set register VF to the least significant bit prior to the shift
                let vx = cpu.registers.get_vn(x);
                let shifted_vx = vx >> 1; 

                cpu.registers.set_vn(x, shifted_vx);

                let lsb = vx & MASK_LSBIT;
                cpu.registers.set_vn(0xF, lsb);

                cpu.program_counter.next();
            }
            _ => panic!("Invalid 0x8xxn instruction"),
        }
    }

    pub fn exec_0xa(cpu: &mut Cpu, addr: u16) {
        // ANNN
        // Sets I to the address NNN
        cpu.registers.set_i(addr);
        cpu.program_counter.next();
    }

    pub fn exec_0xd(cpu: &mut Cpu, n: u8, x: u8, y: u8, ram: &mut Ram, ppu: &mut Ppu) {
        // DXYN
        // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels
        // and a height of N pixels. Each row of 8 pixels is read as bit-coded
        // starting from memory location I; I value does not change after the
        // execution of this instruction. As described above, VF is set to 1
        // if any screen pixels are flipped from set to unset when the sprite
        // is drawn, and to 0 if that does not happen.
        let vx = cpu.registers.get_vn(x);
        let vy = cpu.registers.get_vn(y);
        let i = cpu.registers.get_i();
        let sprite_height = n;
        let sprite_width = 8_u8;
        //
        //clear vf
        cpu.registers.set_vn(0xF, 0);

        let mut pixel_flipped = false;

        for height in 0..sprite_height {
            let byte = ram.read_byte(i + height as u16);
            if ppu.render_pixels(vx, vy, byte, height, sprite_width) {
                pixel_flipped = true;
            }
        }

        if pixel_flipped {
            cpu.registers.set_vn(0xF, 1);
        } else {
            cpu.registers.set_vn(0xF, 0);
        }

        cpu.debug_draw(ppu);

        cpu.program_counter.next();
    }

    pub fn exec_0xe(cpu: &mut Cpu, nn: u8, x: u8) {
        match nn {
            0xA1 => {
                // EXA1
                // Skips the next instruction if the key stored in VX is
                // not pressed (usually the next instruction is a jump to skip a code block).
                let vx = cpu.registers.get_vn(x);
                // first = key 5
                if vx == 5 {
                    cpu.program_counter.next();
                } else {
                    cpu.program_counter.skip_next();
                }
            }
            0x9E => {
                // EX9E
                // Skips the next instruction if the key stored in VX is
                // pressed (usually the next instruction is a jump to skip a code block).
                let vx = cpu.registers.get_vn(x);

                if vx != 4 && vx != 6 {
                    cpu.program_counter.skip_next();
                } else {
                    cpu.program_counter.next();
                }
            }
            _ => panic!("Invalid 0xExnn instruction"),
        }
    }

    pub fn exec_0xf(cpu: &mut Cpu, nn: u8, x: u8, ram: &mut Ram) {
        match nn {
            0x07 => {
                // FX07
                // Sets VX to the value of the delay timer
                cpu.registers.set_vn(x, cpu.get_delay_timer());
                cpu.program_counter.next();
            }
            0x1E => {
                // FX1E
                // Adds VX to I. VF is not affected
                let vx = cpu.registers.get_vn(x);
                let i = cpu.registers.get_i();
                let result = vx as u16 + i;
                cpu.registers.set_i(result);
                cpu.program_counter.next();
            }
            0x15 => {
                // FX15
                // Sets the delay timer to VX
                cpu.set_delay_timer(cpu.registers.get_vn(x));
                cpu.program_counter.next();
            }
            0x18 => {
                // FX18
                // Sets the sound timer to VX
                cpu.program_counter.next();
            }
            0x65 => {
                // FX65
                // Fills from V0 to VX (including VX) with values from memory,
                // starting at address I. The offset from I is increased by 1 for
                // each value read
                for i in 0..=x {
                    let i_reg = cpu.registers.get_i();
                    cpu.registers.set_vn(i, ram.read_byte(i_reg + i as u16));
                    cpu.registers.set_i(i_reg + 1);
                }

                cpu.program_counter.next();
            }
            _ => panic!("Invalid 0xFxnn instruction"),
        }
    }
}
