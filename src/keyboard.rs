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
                    keycode: Some(Keycode::Num1),
                    ..
                } => self.key = Some(1),
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => self.key = Some(2),
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => self.key = Some(3),
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
                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => self.key = Some(7),
                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => self.key = Some(8),
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => self.key = Some(9),
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => self.key = Some(0),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.key = Some(10),
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => self.key = Some(11),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => self.key = Some(12),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.key = Some(13),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => self.key = Some(14),
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => self.key = Some(15),
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                }
                | Event::KeyUp {
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
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num7),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num8),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num9),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Num0),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::B),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => self.key = None,
                _ => {}
            };
        }
    }
}
