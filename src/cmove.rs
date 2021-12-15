use std::fmt::{Display, Error, Formatter};

use crate::{squares::Square, piece::Type};

const VALID_UCI_CHARS: [u8; 8] = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
const VALID_UCI_NUMS: [u8; 8] = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
const VALID_UCI_PROMOTIONS: [u8; 4] = [b'q', b'r', b'b', b'n'];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: Square,
    to: Square,
    capture: Option<Type>,
    promotion: Option<Type>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUndoInfo {
    pub ep_square: u64,
    pub castling_rights: u64,
    pub halfmove_clock: u8,
}

impl MoveUndoInfo {
    pub const fn new(ep_square: u64, castling_rights: u64, halfmove_clock: u8) -> Self {
        Self {
            ep_square,
            castling_rights,
            halfmove_clock,
        }
    }
}

impl Move {
    pub const fn new(from: Square, to: Square, capture: Option<Type>, promotion: Option<Type>) -> Self {
        Self {
            from,
            to,
            capture,
            promotion,
        }
    }

    pub const fn null() -> Self {
        Self { from: Square::A1, to: Square::A1, capture: None, promotion: None }
    }

    pub const fn from_sq(&self) -> Square {
        self.from
    }

    #[allow(clippy::wrong_self_convention)]
    pub const fn to_sq(&self) -> Square {
        self.to
    }

    pub const fn capture(&self) -> Option<Type> {
        self.capture
    }

    pub const fn promotion(&self) -> Option<Type> {
        self.promotion
    }

    fn set_to(&mut self, to: Square) {
        self.to = to;
    }

    fn set_from(&mut self, from: Square) {
        self.from = from;
    }

    pub const fn is_capture(&self) -> bool {
        self.capture.is_some()
    }

    pub const fn is_promotion(&self) -> bool {
        self.promotion.is_some()
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
                b'n' => Some(Type::Knight),
                b'b' => Some(Type::Bishop),
                b'r' => Some(Type::Rook),
                b'q' => Some(Type::Queen),
                _ => unreachable!(),
            }
        } else {
            None
        };
        Ok(Self::new(from.into(), to.into(), None, promotion))
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
        let promo = self.promotion.map_or("", |promo| match promo {
            Type::Knight => "n",
            Type::Bishop => "b",
            Type::Rook => "r",
            Type::Queen => "q",
            _ => panic!("invalid promotion"),
        });
        write!(f, "{}{}{}{}{}", from_file, from_rank, to_file, to_rank, promo)
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} -> {:?}, capture: {:?}, promo: {:?}", self.from_sq(), self.to_sq(), self.capture, self.promotion)
    }
}