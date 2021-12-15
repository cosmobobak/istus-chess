#![allow(clippy::cast_possible_truncation)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    White,
    Black,
}

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
