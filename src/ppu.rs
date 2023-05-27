use crate::ram::Ram;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const MASK_MSBIT: u8 = 0b1000_0000;

pub struct Ppu {
    pub display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    pub pixel_flipped: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            display: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            pixel_flipped: false,
        }
    }

    pub fn clear(&mut self) {
        for height in 0..SCREEN_HEIGHT {
            for width in 0..SCREEN_WIDTH {
                self.display[height][width] = 0;
            }
        }
    }

    pub fn render_pixels(
        &mut self,
        vx: u8,
        vy: u8,
        i: u16,
        sprite_height: u8,
        sprite_width: u8,
        ram: &mut Ram,
    ) {
        for height in 0..sprite_height {
            let mut x = vx as usize % SCREEN_WIDTH;
            let y = (vy + height) as usize % SCREEN_HEIGHT;

            let mut byte = ram.read_byte(i + height as u16);

            if y > SCREEN_HEIGHT - 1 {
                // quit itself if exceed the bottom
                break;
            }

            self.pixel_flipped = false;

            for _ in 0..sprite_width {
                if x > SCREEN_WIDTH - 1 {
                    // continue to skip any pixels off the side
                    continue;
                }

                let bit = (byte & MASK_MSBIT) >> 7;

                if bit == 1 {
                    if self.display[y][x] == 1 {
                        self.pixel_flipped = true;
                        self.display[y][x] = 0
                    } else {
                        self.display[y][x] = 1
                    }
                }

                x += 1;
                byte <<= 1;
            }
        }
    }
}
