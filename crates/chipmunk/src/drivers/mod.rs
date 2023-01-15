#![allow(clippy::needless_bool)]
#![allow(clippy::if_same_then_else)]

mod display;
mod audio;
mod keyboard;

use audio::Sdl2Audio;
use display::Sdl2Display;
use keyboard::Sdl2Keyboard;

use sdl2::{event::Event, EventPump};

use emulator::{Vram, EmulatorError, Platform};

pub struct Sdl2Platform {
    event_pump: EventPump,
    display: display::Sdl2Display,
    audio: Sdl2Audio,
    keyboard: keyboard::Sdl2Keyboard,
}

impl Sdl2Platform {
    pub fn new() -> Self {
        let mut context = sdl2::init().unwrap();
        let event_pump = context.event_pump().unwrap();

        let display = Sdl2Display::new(&mut context);
        let audio = Sdl2Audio::new(&mut context);
        let keyboard = Sdl2Keyboard::new();

        Self {
            event_pump,
            display,
            audio,
            keyboard,
        }
    }
}

impl Platform for Sdl2Platform {
    fn draw(&mut self, vram: Vram) {
        self.display.draw(vram);
    }

    fn start_beep(&mut self) {
        self.audio.start();
    }

    fn stop_beep(&mut self) {
        self.audio.stop();
    }

    fn scan_keys(&mut self) {
        self.keyboard.scan(&mut self.event_pump);
    }

    fn key_is_pressed(&self, key: &u8) -> bool {
        self.keyboard.is_pressed(key)
    }

    fn should_quit(&mut self) -> bool {
        if self.keyboard.escape_is_pressed() {
            true
        } else if matches!(self.event_pump.poll_event(), Some(Event::Quit { .. })) {
            true
        } else {
            false
        }
    }

    fn await_keypress(&mut self) -> Result<u8, EmulatorError> {
        self.keyboard.await_keypress(&mut self.event_pump)
    }
}
