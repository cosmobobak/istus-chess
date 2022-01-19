#![allow(clippy::cast_possible_truncation)]

use std::fmt::{Display, Error, Formatter};

use crate::{squares::Square, piece::PieceType};

const VALID_UCI_CHARS: [u8; 8] = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
const VALID_UCI_NUMS: [u8; 8] = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
const VALID_UCI_PROMOTIONS: [u8; 4] = [b'q', b'r', b'b', b'n'];

/// From Stockfish, moves can be packed into a u16 like so:
/// bit  0- 5: destination square (from 0 to 63)
/// bit  6-11: origin square (from 0 to 63)
/// bit 12-13: promotion piece type - 2 (from KNIGHT-2 to QUEEN-2)
/// bit 14-15: special move flag: promotion (1), en passant (2), castling (3)

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum MoveType {
    Normal,
    Promotion = 1 << 14,
    EnPassant = 2 << 14,
    Castling  = 3 << 14
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUndoInfo {
    pub ep_square: u64,
    pub castling_rights: u64,
    pub halfmove_clock: u8,
    pub captured_piece: PieceType,
}

impl MoveUndoInfo {
    pub const fn new(ep_square: u64, castling_rights: u64, halfmove_clock: u8, captured_piece: PieceType) -> Self {
        Self {
            ep_square,
            castling_rights,
            halfmove_clock,
            captured_piece
        }
    }
}

impl Move {
    pub const fn new(from: Square, to: Square) -> Self {
        Self(((from as u16) << 6) | to as u16)
    }

    pub const fn new_promotion(from: Square, to: Square, pt: PieceType) -> Self {
        Self(((from as u16) << 6) | to as u16 
            | (((pt as u16) - PieceType::Knight as u16) << 12)
            | MoveType::Promotion as u16)
    }

    pub const fn new_castling(from: Square, to: Square) -> Self {
        Self(((from as u16) << 6) | to as u16 | MoveType::Castling as u16)
    }

    pub const fn new_ep(from: Square, to: Square) -> Self {
        Self(((from as u16) << 6) | to as u16 | MoveType::EnPassant as u16)
    }

    pub const fn null() -> Self {
        Self(0)
    }

    #[allow(clippy::wrong_self_convention)]
    pub const fn from_sq(self) -> Square {
        ((self.0 >> 6) & 0x3F) as Square
    }

    #[allow(clippy::wrong_self_convention)]
    pub const fn to_sq(self) -> Square {
        (self.0 & 0x3F) as Square
    }

    pub const fn move_type(self) -> MoveType {
        use MoveType::{Castling, EnPassant, Normal, Promotion};
        match (self.0 & (3 << 14)) >> 14 {
            0 => Normal,
            1 => Promotion,
            2 => EnPassant,
            3 => Castling,
            _ => unreachable!()
        }
    }

    pub const fn promotion(self) -> PieceType {
        use PieceType::{Knight, Queen, Rook, Bishop};
        // ((self.0 >> 12) & 3) + PieceType::Knight as u16
        let val = (self.0 >> 12) & 3;
        match val {
            0 => Knight,
            1 => Bishop,
            2 => Rook,
            3 => Queen,
            _ => unreachable!()
        }
    }

    fn set_to(&mut self, to: Square) {
        self.0 = (self.0 & !0x3F) | (to as u16 & 0x3F);
    }

    fn set_from(&mut self, from: Square) {
        self.0 = (self.0 & !(0x3F << 6)) | ((from as u16 & 0x3F) << 6);
    }

    pub const fn is_promotion(self) -> bool {
        (self.0 & (3 << 14)) != MoveType::Promotion as u16
    }

    pub fn from_uci(uci: &str) -> Result<Self, &'static str> {
        if !(uci.len() == 4 || uci.len() == 5) {
            return Err("uci is of an invalid length");
        }
        let uci = uci.as_bytes();
        if !(uci.len() == 4 || uci.len() == 5) {
            return Err("uci is of an invalid length");
        }
        let (ff, fr, tf, tr) = (uci[0], uci[1], uci[2], uci[3]);

        let chars_valid = VALID_UCI_CHARS.contains(&ff)
            && VALID_UCI_NUMS.contains(&fr)
            && VALID_UCI_CHARS.contains(&tf)
            && VALID_UCI_NUMS.contains(&tr)
            && VALID_UCI_PROMOTIONS.contains(uci.get(4).unwrap_or(&b'n'));

        if !chars_valid {
            return Err("uci contains invalid characters");
        }

        let from_file = uci[0] - b'a';
        let from_rank = uci[1] - b'1';
        let to_file = uci[2] - b'a';
        let to_rank = uci[3] - b'1';
        // a1 = 0, b1 = 1, ..., h1 = 7, a2 = 8, ..., h8 = 63
        let from = (from_file as usize) + (from_rank as usize) * 8;
        let to = (to_file as usize) + (to_rank as usize) * 8;
        
        let promotion = if uci.len() == 5 {
            if to_rank != 0 && to_rank != 7 {
                return Err("uci contains invalid promotion");
            }
            match uci[4] {
                b'n' => PieceType::Knight,
                b'b' => PieceType::Bishop,
                b'r' => PieceType::Rook,
                b'q' => PieceType::Queen,
                _ => unreachable!(),
            }
        } else {
            PieceType::None
        };
        Ok(Self::new_promotion(from, to, promotion))
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        const FILES: &[u8; 8] = b"abcdefgh";
        const RANKS: &[u8; 8] = b"12345678";
        let from_file = FILES[self.from_sq() as usize % 8] as char;
        let from_rank = RANKS[self.from_sq() as usize / 8] as char;
        let to_file = FILES[self.to_sq() as usize % 8] as char;
        let to_rank = RANKS[self.to_sq() as usize / 8] as char;
        let promo = match self.promotion() {
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            _ => panic!("invalid promotion"),
        };
        write!(f, "{}{}{}{}{}", from_file, from_rank, to_file, to_rank, promo)
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} -> {:?}, type: {:?}, promo: {:?}", self.from_sq(), self.to_sq(), self.move_type(), self.promotion())
    }
}

#[cfg(test)]
mod move_tests {
    use crate::cmove::Move;
    use crate::piece::PieceType;
    use crate::squares::SquareEnum;
    use SquareEnum::{A7, A8, E2, E4};

    #[test]
    fn uci() {
        let m = Move::from_uci("e2e4").unwrap();
        let from = m.from_sq();
        let to = m.to_sq();
        assert_eq!(from, E2 as usize);
        assert_eq!(to, E4 as usize);
    }

    #[test]
    fn uci_promo() {
        let m = Move::from_uci("a7a8q").unwrap();
        let from = m.from_sq();
        let to = m.to_sq();
        assert_eq!(from, A7 as usize);
        assert_eq!(to, A8 as usize);
        assert_eq!(m.promotion(), PieceType::Queen);
    }

    #[test]
    fn uci_invalid() {
        let m = Move::from_uci("e2e4e5");
        assert!(m.is_err());
        let m = Move::from_uci("e2e9");
        assert!(m.is_err());
        let m = Move::from_uci("2e4e");
        assert!(m.is_err());
        let m = Move::from_uci("e2e4q");
        assert!(m.is_err());
        let m = Move::from_uci("j2e4");
        assert!(m.is_err());
    }
}