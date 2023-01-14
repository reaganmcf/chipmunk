use crate::error::EmulatorError;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
// TODO bool should be replaced with u8's and bitwise ops
pub type Vram = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

pub trait Platform {
    fn draw(&mut self, vram: Vram);

    fn start_beep(&mut self);
    fn stop_beep(&mut self);

    fn scan_keys(&mut self);
    fn key_is_pressed(&self, key: &u8) -> bool;
    fn should_quit(&mut self) -> bool;
    fn await_keypress(&mut self) -> Result<u8, EmulatorError>;
}
