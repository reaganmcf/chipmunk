use std::collections::HashSet;

use chipmunk_backend::EmulatorError;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

const ESCAPE_KEY: u8 = 0xFF;

fn map_keycode(code: Option<Keycode>) -> Option<u8> {
    match code {
        Some(Keycode::Escape) => Some(ESCAPE_KEY),
        Some(Keycode::Num1) => Some(0x1),
        Some(Keycode::Num2) => Some(0x2),
        Some(Keycode::Num3) => Some(0x3),
        Some(Keycode::Num4) => Some(0xC),
        Some(Keycode::Q) => Some(0x4),
        Some(Keycode::W) => Some(0x5),
        Some(Keycode::E) => Some(0x6),
        Some(Keycode::R) => Some(0xD),
        Some(Keycode::A) => Some(0x7),
        Some(Keycode::S) => Some(0x8),
        Some(Keycode::D) => Some(0x9),
        Some(Keycode::F) => Some(0xE),
        Some(Keycode::Z) => Some(0xA),
        Some(Keycode::X) => Some(0x0),
        Some(Keycode::C) => Some(0xB),
        Some(Keycode::V) => Some(0xF),
        _ => None,
    }
}

pub struct Sdl2Keyboard {
    pressed_keys: HashSet<u8>,
}

impl Sdl2Keyboard {
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

    pub fn await_keypress(&self, event_pump: &mut EventPump) -> Result<u8, EmulatorError> {
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
