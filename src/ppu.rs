const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const MASK_MSBIT: u8 = 0b1000_0000;

pub struct Ppu {
    pub display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            display: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
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
        mut byte: u8,
        height: u8,
        sprite_width: u8,
    ) -> bool {
        let mut x = vx as usize;
        let mut y = (vy + height) as usize;

        let mut pixel_flipped = false;

        for _ in 0..sprite_width {
            x %= SCREEN_WIDTH;
            y %= SCREEN_HEIGHT;

            let bit = (byte & MASK_MSBIT) >> 7;

            if bit == 1 {
                if self.display[y][x] == 1 {
                    pixel_flipped = true;
                    self.display[y][x] = 0
                } else {
                    self.display[y][x] = 1
                }
            }

            x += 1;
            byte <<= 1;
        }

        pixel_flipped
    }
}
