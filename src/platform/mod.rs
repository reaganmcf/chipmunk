use crate::{emulator::VRAM, error::EmulatorError};

mod sdl2;

pub use self::sdl2::Sdl2Platform;

pub trait Platform {
    fn draw(&mut self, vram: VRAM);

    fn start_beep(&mut self);
    fn stop_beep(&mut self);

    fn scan_keys(&mut self);
    fn key_is_pressed(&self, key: &u8) -> bool;
    fn should_quit(&mut self) -> bool;
    fn await_keypress(&mut self) -> Result<u8, EmulatorError>;
}
