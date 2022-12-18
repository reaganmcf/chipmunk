use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

use crate::error::EmulatorError;

pub struct Keyboard {
    pressed_keys: Vec<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::with_capacity(16),
        }
    }

    pub fn await_keypress(event_pump: &mut EventPump) -> Result<u8, EmulatorError> {
        loop {
            match event_pump.wait_event() {
                Event::Quit { .. } => return Err(EmulatorError::Exit),
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => return Err(EmulatorError::Exit),
                    Some(Keycode::Num0) => return Ok(0x0),
                    Some(Keycode::Num1) => return Ok(0x1),
                    Some(Keycode::Num2) => return Ok(0x2),
                    Some(Keycode::Num3) => return Ok(0x3),
                    Some(Keycode::Num4) => return Ok(0x4),
                    Some(Keycode::Num5) => return Ok(0x5),
                    Some(Keycode::Num6) => return Ok(0x6),
                    Some(Keycode::Num7) => return Ok(0x7),
                    Some(Keycode::Num8) => return Ok(0x8),
                    Some(Keycode::Num9) => return Ok(0x9),
                    Some(Keycode::A) => return Ok(0xA),
                    Some(Keycode::B) => return Ok(0xB),
                    Some(Keycode::C) => return Ok(0xC),
                    Some(Keycode::D) => return Ok(0xD),
                    Some(Keycode::E) => return Ok(0xE),
                    Some(Keycode::F) => return Ok(0xF),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn scan(&mut self, event_pump: &mut EventPump) {
        self.pressed_keys.clear();
        for event in event_pump.poll_iter() {
            if let Event::KeyDown { keycode, .. } = event {
                let pressed_key = match keycode {
                    Some(Keycode::Escape) => Some(0xFF),
                    Some(Keycode::Num0) => Some(0x0),
                    Some(Keycode::Num1) => Some(0x1),
                    Some(Keycode::Num2) => Some(0x2),
                    Some(Keycode::Num3) => Some(0x3),
                    Some(Keycode::Num4) => Some(0x4),
                    Some(Keycode::Num5) => Some(0x5),
                    Some(Keycode::Num6) => Some(0x6),
                    Some(Keycode::Num7) => Some(0x7),
                    Some(Keycode::Num8) => Some(0x8),
                    Some(Keycode::Num9) => Some(0x9),
                    Some(Keycode::A) => Some(0xA),
                    Some(Keycode::B) => Some(0xB),
                    Some(Keycode::C) => Some(0xC),
                    Some(Keycode::D) => Some(0xD),
                    Some(Keycode::E) => Some(0xE),
                    Some(Keycode::F) => Some(0xF),
                    _ => None,
                };

                if let Some(pressed_key) = pressed_key {
                    self.pressed_keys.push(pressed_key);
                }
            }
        }
    }
    
    pub fn is_pressed(&self, key: &u8) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn escape_is_pressed(&self) -> bool {
        self.is_pressed(&0xFF)
    }
}
