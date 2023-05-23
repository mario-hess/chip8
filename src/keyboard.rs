use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Keyboard {
    pub key: Option<Keycode>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { key: None }
    }

    pub fn set_key(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.key = Some(Keycode::Escape),
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => self.key = Some(Keycode::Num4),
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => self.key = None,
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => self.key = Some(Keycode::Num5),
                Event::KeyUp {
                    keycode: Some(Keycode::Num5),
                    ..
                } => self.key = None,
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => self.key = Some(Keycode::Num6),
                Event::KeyUp {
                    keycode: Some(Keycode::Num6),
                    ..
                } => self.key = None,
                _ => {}
            };
        }
    }
}
