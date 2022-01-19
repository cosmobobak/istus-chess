use crate::{piece::Type, bitmethods::Bithackable};
use crate::magicnumbers::{BB_B1, BB_B8, BB_C1, BB_C8, BB_CORNERS, BB_D1, BB_D8, BB_E1, BB_E8, BB_EMPTY, BB_F1, BB_F8, BB_G1, BB_G8, BB_RANK_1, BB_RANK_2, BB_RANK_7, BB_RANK_8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub castling_rights: u64,
    pub ep_square: u64,
    pub occupied_co: [u64; 2],
}

impl Bitboard {
    pub fn reset(&mut self) {
        self.pawns = BB_RANK_2 | BB_RANK_7;
        self.knights = BB_B1 | BB_G1 | BB_B8 | BB_G8;
        self.bishops = BB_C1 | BB_F1 | BB_C8 | BB_F8;
        self.rooks = BB_CORNERS;
        self.queens = BB_D1 | BB_D8;
        self.kings = BB_E1 | BB_E8;
        self.castling_rights = BB_CORNERS;
        self.ep_square = BB_EMPTY;
        self.occupied_co[0] = BB_RANK_1 | BB_RANK_2;
        self.occupied_co[1] = BB_RANK_7 | BB_RANK_8;
    }

    pub const fn new() -> Self {
        Self {
            pawns: BB_RANK_2 | BB_RANK_7,
            knights: BB_B1 | BB_G1 | BB_B8 | BB_G8,
            bishops: BB_C1 | BB_F1 | BB_C8 | BB_F8,
            rooks: BB_CORNERS,
            queens: BB_D1 | BB_D8,
            kings: BB_E1 | BB_E8,
            castling_rights: BB_CORNERS,
            ep_square: BB_EMPTY,
            occupied_co: [BB_RANK_1 | BB_RANK_2, BB_RANK_7 | BB_RANK_8],
        }
    }

    pub const fn clear() -> Self {
        Self {
            pawns: BB_EMPTY,
            knights: BB_EMPTY,
            bishops: BB_EMPTY,
            rooks: BB_EMPTY,
            queens: BB_EMPTY,
            kings: BB_EMPTY,
            castling_rights: BB_EMPTY,
            ep_square: BB_EMPTY,
            occupied_co: [BB_EMPTY, BB_EMPTY],
        }
    }

    pub fn piece_type_at(&self, square: usize) -> Option<Type> {
        // feel free to optimize this
        if !(self.occupied_co[0] | self.occupied_co[1]).test(square) {
            None
        } else if self.pawns.test(square) {
            Some(Type::Pawn)
        } else if self.knights.test(square) {
            Some(Type::Knight)
        } else if self.bishops.test(square) {
            Some(Type::Bishop)
        } else if self.rooks.test(square) {
            Some(Type::Rook)
        } else if self.queens.test(square) {
            Some(Type::Queen)
        } else {
            Some(Type::King)
        }
    }

    #[inline]
    pub const fn occupied(&self) -> u64 {
        self.occupied_co[0] | self.occupied_co[1]
    }
}