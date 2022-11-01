use crate::{registers::Reg, error::EmulatorError};

#[derive(Debug)]
pub enum OpCode {
    SetVX { register: Reg, value: u8 },
    NoOp,
}



impl TryInto<OpCode> for u16 {
    type Error = EmulatorError;
    fn try_into(self) -> Result<OpCode, Self::Error> {
        let parts = stretch_u16(self);
        match parts {
            [0x06, x, n1, n2] => {
                let value = (n1 << 4) | n2;
                let register = x.into();

                return Ok(OpCode::SetVX { register, value });
            }
            _ => return Err(EmulatorError::UnknownOpCode),
        }
    }
}

// Turn u16 into 4 u8s (but really u4s, since the first half is always 0), by stretching them
// Ex: 0x6278 -> [0x06, 0x02, 0x07, 0x08]
fn stretch_u16(input: u16) -> [u8; 4] {
    let res = [0, 0, 0, 0];

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
