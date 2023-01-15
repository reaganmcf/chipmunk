use crate::{error::EmulatorError, Vram};


pub trait Platform {
    fn draw(&mut self, vram: Vram);

    fn start_beep(&mut self);
    fn stop_beep(&mut self);

    fn scan_keys(&mut self);
    fn key_is_pressed(&self, key: &u8) -> bool;
    fn should_quit(&mut self) -> bool;
    fn await_keypress(&mut self) -> Result<u8, EmulatorError>;
}
