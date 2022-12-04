use crate::{error::EmulatorError, registers::Reg};
#[derive(Debug)]
pub enum OpCode {
    _00E0,
    _00EE,
    _1NNN(u16),
    _2NNN(u16),
    _3XNN { reg: Reg, value: u8 },
    _4XNN { reg: Reg, value: u8 },
    _6XNN { reg: Reg, value: u8 },
    _7XNN { reg: Reg, value: u8 },
    _8XY0 { x: Reg, y: Reg },
    _8XY2 { x: Reg, y: Reg },
    _8XY4 { x: Reg, y: Reg },
    _8XY5 { x: Reg, y: Reg },
    ANNN(u16),
    CXNN { reg: Reg, value: u8 },
    DXYN { x: Reg, y: Reg, height: u8 },
    EXA1(Reg),
    FX07(Reg),
    FX0A(Reg),
    FX15(Reg),
    FX18(u8),
    FX1E(Reg),
    FX29(Reg),
    FX33(Reg),
    FX65(Reg),
}

// TODO Finish these
impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::_00E0 => f.write_str("Clears the screen"),
            OpCode::_00EE => f.write_str("Returns from a subroutine"),
            OpCode::_1NNN(nnn) => f.write_str(&format!("Jumps to address {}", nnn)),
            OpCode::_2NNN(nnn) => f.write_str(&format!("Calls subroutine at {}", nnn)),
            OpCode::_3XNN { reg, value } => f.write_str(&format!("if ({:?} == {})", reg, value)),
            _ => f.write_str(&format!("Don't know what {:#?} is", self)),
        }
    }
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
            [0x8, x, y, 0x2] => {
                let x = x.into();
                let y = y.into();

                Ok(OpCode::_8XY2 { x, y })
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
            [0xa, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::ANNN(nnn))
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
                let value = x;

                Ok(OpCode::FX18(value))
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
            [0xf, x, 0x6, 0x5] => {
                let reg = x.into();
                Ok(OpCode::FX65(reg))
            }
            _ => Err(EmulatorError::UnknownOpCode(format!("{:x}", self))),
        }
    }
}

// Turn u16 into 4 u8s (but really u4s, since the first half is always 0), by stretching them
// Ex: 0x6278 -> [0x06, 0x02, 0x07, 0x08]
fn stretch_u16(input: u16) -> [u8; 4] {
    let fourth = (input & 0x000F) as u8;
    let third = ((input & 0x00F0) >> 4) as u8;
    let second = ((input & 0x0F00) >> 8) as u8;
    let first = ((input & 0xF000) >> 12) as u8;

    [first, second, third, fourth]
}

#[cfg(test)]
mod tests {
    use super::stretch_u16;

    #[test]
    fn simple() {
        let actual = stretch_u16(0x6278);
        assert_eq!(actual, [0x06 as u8, 0x02, 0x07, 0x08]);
    }
}
