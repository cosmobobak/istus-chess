#![allow(clippy::cast_possible_truncation)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    White,
    Black,
}

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

impl From<u8> for Colour {
    fn from(colour: u8) -> Self {
        unsafe { std::mem::transmute(colour & 1) }
    }
}

impl From<u16> for Colour {
    fn from(colour: u16) -> Self {
        let byte = colour as u8;
        unsafe { std::mem::transmute(byte & 1) }
    }
}

impl From<u32> for Colour {
    fn from(colour: u32) -> Self {
        let byte = colour as u8;
        unsafe { std::mem::transmute(byte & 1) }
    }
}


#[cfg(test)]
mod colour_tests {
    use crate::colour::Colour;
    use Colour::{Black, White};

    #[test]
    fn colour_creation() {
        let byte = 0_u8;
        assert_eq!(White, byte.into());
        let short = 0_u16;
        assert_eq!(White, short.into());
        let int = 0_u32;
        assert_eq!(White, int.into());

        let byte = 1_u8;
        assert_eq!(Black, byte.into());
        let short = 1_u16;
        assert_eq!(Black, short.into());
        let int = 1_u32;
        assert_eq!(Black, int.into());
    }

    #[test]
    fn colour_modulus() {
        let byte = 160_u8;
        assert_eq!(White, byte.into());
        let short = 160_u16;
        assert_eq!(White, short.into());
        let int = 160_u32;
        assert_eq!(White, int.into());

        let byte = 161_u8;
        assert_eq!(Black, byte.into());
        let short = 161_u16;
        assert_eq!(Black, short.into());
        let int = 161_u32;
        assert_eq!(Black, int.into());
    }

    #[test]
    fn colour_to_int() {
        let white = White;
        assert_eq!(0, white as u32);

        let black = Black;
        assert_eq!(1, black as u32);
    }
}