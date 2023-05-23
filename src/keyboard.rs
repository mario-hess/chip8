use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Keyboard {
    pub key: Option<u8>,
    pub escape_pressed: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key: None,
            escape_pressed: false,
        }
    }

    pub fn set_key(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.escape_pressed = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => self.key = Some(4),
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => self.key = Some(5),
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => self.key = Some(6),
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num5),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num6),
                    ..
                } => self.key = None,
                _ => {}
            };
        }
    }
}
