mod emulator;
mod error;
mod opcode;
mod platform;
mod registers;
mod utils;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
// TODO bool should be replaced with u8's and bitwise ops
pub type Vram = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

pub use error::EmulatorError;
use opcode::OpCode;
pub use platform::Platform;

pub fn run(rom: Vec<u8>, platform: Box<dyn Platform>, debug: bool) {
    let mut emulator = emulator::Emulator::new(rom, platform, debug);
    emulator.start();
}

pub fn disassemble(rom: Vec<u8>) -> Result<Vec<OpCode>, EmulatorError> {
    let mut idx = 0;
    let mut res = Vec::new();
    while idx < rom.len() {
        let first_half = rom[idx];
        let second_half = rom[idx + 0x1];
        idx += 2;
        let raw_opcode: u16 = u16::from_be_bytes([first_half, second_half]);

        res.push(raw_opcode.try_into()?);
    }

    Ok(res)
}
