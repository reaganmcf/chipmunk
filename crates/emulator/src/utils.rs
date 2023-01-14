/// Get the binary decoded decimal from a u8
///
/// Example:
/// ```
/// bcd(104) == [1, 0, 4];
/// bcd(0xFA) == [2, 5, 0];
/// ```
pub fn bcd(num: u8) -> [u8; 3] {
    let mut curr = num;
    let mut result = [0, 0, 0];
    for i in 0..3 {
        let digit = curr % 10;
        result[2 - i] = digit as u8;
        curr /= 10;
    }

    result
}

/// Turn u16 into 4 u8s (but really u4s, since the first half is always 0), by stretching them
///
/// Example:
/// ```
/// stretch_u16(0x6278) == [0x06, 0x02, 0x07, 0x08];
/// ```
pub fn stretch_u16(input: u16) -> [u8; 4] {
    let fourth = (input & 0x000F) as u8;
    let third = ((input & 0x00F0) >> 4) as u8;
    let second = ((input & 0x0F00) >> 8) as u8;
    let first = ((input & 0xF000) >> 12) as u8;

    [first, second, third, fourth]
}

#[cfg(test)]
mod tests {
    use crate::utils::{bcd, stretch_u16};

    #[test]
    fn bcd_works() {
        assert_eq!(bcd(104), [1, 0, 4]);
        assert_eq!(bcd(0xFA), [2, 5, 0])
    }

    #[test]
    fn simple() {
        let actual = stretch_u16(0x6278);
        assert_eq!(actual, [0x06 as u8, 0x02, 0x07, 0x08]);
    }
}
