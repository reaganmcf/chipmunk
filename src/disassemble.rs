use crate::{error::EmulatorError, opcode::OpCode};

pub struct Disassembler();

impl Disassembler {
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
}
