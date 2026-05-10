use crate::utils::bit_magic::{concat_bytes, get_bit_from_u8, nibble_from_bits, unconcat_bytes};

pub struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

/// conversions
impl Flags {
    pub const fn bits(&self) -> [bool; 4] {
        [self.zero, self.subtract, self.half_carry, self.carry]
    }

    pub const fn as_u8(&self) -> u8 {
        nibble_from_bits(self.bits()) << 4
    }

    pub const fn from_u8(b: u8) -> Self {
        Self {
            zero: get_bit_from_u8(b, 0),
            subtract: get_bit_from_u8(b, 1),
            half_carry: get_bit_from_u8(b, 2),
            carry: get_bit_from_u8(b, 3),
        }
    }
}

impl core::convert::From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        value.as_u8()
    }
}

impl core::convert::From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

/// combo register methods
impl Registers {
    pub const fn af(&self) -> u16 {
        concat_bytes([self.a, self.f.as_u8()])
    }

    pub const fn set_af(&mut self, value: u16) {
        let [a, f] = unconcat_bytes(value);
        self.a = a;
        self.f = Flags::from_u8(f);
    }

    pub const fn bc(&self) -> u16 {
        concat_bytes([self.b, self.c])
    }

    pub const fn set_bc(&mut self, value: u16) {
        [self.b, self.c] = unconcat_bytes(value);
    }

    pub const fn de(&self) -> u16 {
        concat_bytes([self.b, self.c])
    }

    pub const fn set_de(&mut self, value: u16) {
        [self.b, self.c] = unconcat_bytes(value);
    }

    pub const fn hl(&self) -> u16 {
        concat_bytes([self.h, self.l])
    }

    pub const fn set_hl(&mut self, value: u16) {
        [self.h, self.l] = unconcat_bytes(value);
    }
}
