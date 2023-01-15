#![allow(clippy::upper_case_acronyms)]

use crate::{error::EmulatorError, registers::Reg, utils::stretch_u16};
#[derive(Debug)]
pub enum OpCode {
    _00E0,
    _00EE,
    _1NNN(u16),
    _2NNN(u16),
    _3XNN { reg: Reg, value: u8 },
    _4XNN { reg: Reg, value: u8 },
    _5XY0 { x: Reg, y: Reg },
    _6XNN { reg: Reg, value: u8 },
    _7XNN { reg: Reg, value: u8 },
    _8XY0 { x: Reg, y: Reg },
    _8XY1 { x: Reg, y: Reg },
    _8XY2 { x: Reg, y: Reg },
    _8XY3 { x: Reg, y: Reg },
    _8XY4 { x: Reg, y: Reg },
    _8XY5 { x: Reg, y: Reg },
    _8XY6 { x: Reg, y: Reg },
    _8XY7 { x: Reg, y: Reg },
    _8XYE { x: Reg, y: Reg },
    _9XY0 { x: Reg, y: Reg },
    ANNN(u16),
    BNNN(u16),
    CXNN { reg: Reg, value: u8 },
    DXYN { x: Reg, y: Reg, height: u8 },
    EX9E(Reg),
    EXA1(Reg),
    FX07(Reg),
    FX0A(Reg),
    FX15(Reg),
    FX18(Reg),
    FX1E(Reg),
    FX29(Reg),
    FX33(Reg),
    FX55(Reg),
    FX65(Reg),
}

impl TryInto<OpCode> for u16 {
    type Error = EmulatorError;
    fn try_into(self) -> Result<OpCode, Self::Error> {
        let parts = stretch_u16(self);

        println!("raw code = {:?}", parts);
        match parts {
            [0x0, 0x0, 0xe, 0x0] => Ok(OpCode::_00E0),
            [0x0, 0x0, 0xe, 0xe] => Ok(OpCode::_00EE),
            [0x1, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::_1NNN(nnn))
            }
            [0x2, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::_2NNN(nnn))
            }
            [0x3, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let reg = x.into();

                Ok(OpCode::_3XNN { reg, value })
            }
            [0x4, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let reg = x.into();

                Ok(OpCode::_4XNN { reg, value })
            }
            [0x5, x, y, 0x0] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_5XY0 { x, y })
            }
            [0x6, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let reg = x.into();

                Ok(OpCode::_6XNN { reg, value })
            }
            [0x7, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let reg = x.into();

                Ok(OpCode::_7XNN { reg, value })
            }
            [0x8, x, y, 0x0] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY0 { x, y })
            }
            [0x8, x, y, 0x1] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY1 { x, y })
            }
            [0x8, x, y, 0x2] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY2 { x, y })
            }
            [0x8, x, y, 0x3] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY3 { x, y })
            }
            [0x8, x, y, 0x4] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY4 { x, y })
            }
            [0x8, x, y, 0x5] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY5 { x, y })
            }
            [0x8, x, y, 0x6] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY6 { x, y })
            }
            [0x8, x,  y, 0x7] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY7 { x, y })
            }
            [0x8, x, y, 0xe] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XYE { x, y })
            }
            [0x9, x, y, 0] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_9XY0 { x, y })
            }
            [0xa, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::ANNN(nnn))
            }
            [0xb, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::BNNN(nnn))
            }
            [0xe, x, 0x9, 0xe] => {
                let reg = x.into();

                Ok(OpCode::EX9E(reg))
            }
            [0xe, x, 0xa, 0x1] => {
                let reg = x.into();

                Ok(OpCode::EXA1(reg))
            }
            [0xc, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let reg = x.into();

                Ok(OpCode::CXNN { reg, value })
            }
            [0xd, x, y, n] => {
                let x = x.into();
                let y = y.into();
                let height = n;

                Ok(OpCode::DXYN { x, y, height })
            }
            [0xf, x, 0x0, 0x7] => {
                let reg = x.into();

                Ok(OpCode::FX07(reg))
            }
            [0xf, x, 0x0, 0xa] => {
                let dest = x.into();

                Ok(OpCode::FX0A(dest))
            }
            [0xf, x, 0x1, 0x5] => {
                let reg = x.into();
                Ok(OpCode::FX15(reg))
            }
            [0xf, x, 0x1, 0x8] => {
                let reg = x.into();

                Ok(OpCode::FX18(reg))
            }
            [0xf, x, 0x1, 0xe] => {
                let reg = x.into();

                Ok(OpCode::FX1E(reg))
            }
            [0xf, x, 0x2, 0x9] => {
                let reg = x.into();
                Ok(OpCode::FX29(reg))
            }
            [0xf, x, 0x3, 0x3] => {
                let reg = x.into();
                Ok(OpCode::FX33(reg))
            }
            [0xf, x, 0x5, 0x5] => {
                let reg = x.into();
                Ok(OpCode::FX55(reg))
            }
            [0xf, x, 0x6, 0x5] => {
                let reg = x.into();
                Ok(OpCode::FX65(reg))
            }
            _ => Err(EmulatorError::UnknownOpCode(format!("{:x}", self))),
        }
    }
}
