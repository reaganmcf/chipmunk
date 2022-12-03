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
        curr = curr / 10;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::bcd;

    #[test]
    fn bcd_works() {
        assert_eq!(bcd(104), [1, 0, 4]);
        assert_eq!(bcd(0xFA), [2, 5, 0])
    }
}
