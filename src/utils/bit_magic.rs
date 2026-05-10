pub const fn concat_bytes([left, right]: [u8; 2]) -> u16 {
    (left as u16) << 8 | (right as u16)
}

pub const fn unconcat_bytes(x: u16) -> [u8; 2] {
    [((x & 0xFF00) >> 8) as u8, (x & 0x00FF) as u8]
}

pub const fn nibble_from_bits([msb, b1, b2, lsb]: [bool; 4]) -> u8 {
    (msb as u8) << 3 | (b1 as u8) << 2 | (b2 as u8) << 1 | (lsb as u8) << 0
}

/// `bits : 1 1 1 1 1 1 1 1`
/// 
/// `index: 0 1 2 3 4 5 6 7`
pub const fn get_bit_from_u8(b: u8, index: u8) -> bool {
    debug_assert!(index < 8);
    b & (0b1000_0000 >> index) != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_concat_bytes() {
        assert_eq!(concat_bytes([0xAB, 0xCD]), 0xABCD)
    }

    #[test]
    fn test_unconcat_bytes() {
        assert_eq!(unconcat_bytes(0xABCD), [0xAB, 0xCD])
    }

    #[test]
    fn test_nibble_from_bits() {
        assert_eq!(nibble_from_bits([true, true, true, true]), 0b1111);
        assert_eq!(nibble_from_bits([true, true, true, true]) << 4, 0b11110000);
    }
}
