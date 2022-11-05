use crate::{error::EmulatorError, registers::Reg};

#[derive(Debug)]
pub enum OpCode {
    _00E0,
    _1NNN(u16),
    _6XNN { register: Reg, value: u8 },
    ANNN(u16),
    DXYN { x: Reg, y: Reg, height: u8 },
    FX0A(Reg),
    FX18(u8),
    FX29(Reg),
}

impl TryInto<OpCode> for u16 {
    type Error = EmulatorError;
    fn try_into(self) -> Result<OpCode, Self::Error> {
        let parts = stretch_u16(self);
        match parts {
            [0x0, 0x0, 0xe, 0x0] => Ok(OpCode::_00E0),
            [0x1, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::_1NNN(nnn))
            }
            [0x6, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let register = x.into();

                Ok(OpCode::_6XNN { register, value })
            }
            [0xa, n1, n2, n3] => {
                let nnn = ((n1 as u16) << 8) | ((n2 as u16) << 4) | n3 as u16;

                Ok(OpCode::ANNN(nnn))
            }
            [0xd, x, y, n] => {
                let x = x.into();
                let y = y.into();
                let height = n;

                Ok(OpCode::DXYN { x, y, height })
            }
            [0xf, x, 0x0, 0xa] => {
                let dest = x.into();

                Ok(OpCode::FX0A(dest))
            }
            [0xf, x, 0x1, 0x8] => {
                let value = x;

                Ok(OpCode::FX18(value))
            }
            [0xf, x, 0x2, 0x9] => {
                let reg = x.into();
                Ok(OpCode::FX29(reg))
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
