use std::collections::HashSet;

use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

use crate::error::EmulatorError;

const ESCAPE_KEY: u8 = 0xFF;

fn map_keycode(code: Option<Keycode>) -> Option<u8> {
    match code {
        Some(Keycode::Escape) => Some(ESCAPE_KEY),
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
    }
}

pub struct Keyboard {
    pressed_keys: HashSet<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::with_capacity(16),
        }
    }

    pub fn scan(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    let pressed_key = map_keycode(keycode);

                    if let Some(pressed_key) = pressed_key {
                        self.pressed_keys.insert(pressed_key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    let pressed_key = map_keycode(keycode);

                    if let Some(key) = pressed_key {
                        if self.pressed_keys.contains(&key) {
                            self.pressed_keys.remove(&key);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn is_pressed(&self, key: &u8) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn escape_is_pressed(&self) -> bool {
        self.is_pressed(&0xFF)
    }

    pub fn await_keypress(event_pump: &mut EventPump) -> Result<u8, EmulatorError> {
        loop {
            match event_pump.wait_event() {
                Event::Quit { .. } => return Err(EmulatorError::Exit),
                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = map_keycode(keycode) {
                        // Check for escape
                        if keycode == ESCAPE_KEY {
                            return Err(EmulatorError::Exit);
                        } else {
                            return Ok(keycode);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
