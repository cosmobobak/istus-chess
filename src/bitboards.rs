use crate::{piece::Type, bitmethods::Bithackable};

pub const BB_A1: u64 = 1_u64 << 0;
pub const BB_B1: u64 = 1_u64 << 1;
pub const BB_C1: u64 = 1_u64 << 2;
pub const BB_D1: u64 = 1_u64 << 3;
pub const BB_E1: u64 = 1_u64 << 4;
pub const BB_F1: u64 = 1_u64 << 5;
pub const BB_G1: u64 = 1_u64 << 6;
pub const BB_H1: u64 = 1_u64 << 7;
pub const BB_A2: u64 = 1_u64 << 8;
pub const BB_B2: u64 = 1_u64 << 9;
pub const BB_C2: u64 = 1_u64 << 10;
pub const BB_D2: u64 = 1_u64 << 11;
pub const BB_E2: u64 = 1_u64 << 12;
pub const BB_F2: u64 = 1_u64 << 13;
pub const BB_G2: u64 = 1_u64 << 14;
pub const BB_H2: u64 = 1_u64 << 15;
pub const BB_A3: u64 = 1_u64 << 16;
pub const BB_B3: u64 = 1_u64 << 17;
pub const BB_C3: u64 = 1_u64 << 18;
pub const BB_D3: u64 = 1_u64 << 19;
pub const BB_E3: u64 = 1_u64 << 20;
pub const BB_F3: u64 = 1_u64 << 21;
pub const BB_G3: u64 = 1_u64 << 22;
pub const BB_H3: u64 = 1_u64 << 23;
pub const BB_A4: u64 = 1_u64 << 24;
pub const BB_B4: u64 = 1_u64 << 25;
pub const BB_C4: u64 = 1_u64 << 26;
pub const BB_D4: u64 = 1_u64 << 27;
pub const BB_E4: u64 = 1_u64 << 28;
pub const BB_F4: u64 = 1_u64 << 29;
pub const BB_G4: u64 = 1_u64 << 30;
pub const BB_H4: u64 = 1_u64 << 31;
pub const BB_A5: u64 = 1_u64 << 32;
pub const BB_B5: u64 = 1_u64 << 33;
pub const BB_C5: u64 = 1_u64 << 34;
pub const BB_D5: u64 = 1_u64 << 35;
pub const BB_E5: u64 = 1_u64 << 36;
pub const BB_F5: u64 = 1_u64 << 37;
pub const BB_G5: u64 = 1_u64 << 38;
pub const BB_H5: u64 = 1_u64 << 39;
pub const BB_A6: u64 = 1_u64 << 40;
pub const BB_B6: u64 = 1_u64 << 41;
pub const BB_C6: u64 = 1_u64 << 42;
pub const BB_D6: u64 = 1_u64 << 43;
pub const BB_E6: u64 = 1_u64 << 44;
pub const BB_F6: u64 = 1_u64 << 45;
pub const BB_G6: u64 = 1_u64 << 46;
pub const BB_H6: u64 = 1_u64 << 47;
pub const BB_A7: u64 = 1_u64 << 48;
pub const BB_B7: u64 = 1_u64 << 49;
pub const BB_C7: u64 = 1_u64 << 50;
pub const BB_D7: u64 = 1_u64 << 51;
pub const BB_E7: u64 = 1_u64 << 52;
pub const BB_F7: u64 = 1_u64 << 53;
pub const BB_G7: u64 = 1_u64 << 54;
pub const BB_H7: u64 = 1_u64 << 55;
pub const BB_A8: u64 = 1_u64 << 56;
pub const BB_B8: u64 = 1_u64 << 57;
pub const BB_C8: u64 = 1_u64 << 58;
pub const BB_D8: u64 = 1_u64 << 59;
pub const BB_E8: u64 = 1_u64 << 60;
pub const BB_F8: u64 = 1_u64 << 61;
pub const BB_G8: u64 = 1_u64 << 62;
pub const BB_H8: u64 = 1_u64 << 63;

pub const BB_RANK_1: u64 = BB_A1 | BB_B1 | BB_C1 | BB_D1 | BB_E1 | BB_F1 | BB_G1 | BB_H1;
pub const BB_RANK_2: u64 = BB_A2 | BB_B2 | BB_C2 | BB_D2 | BB_E2 | BB_F2 | BB_G2 | BB_H2;
pub const BB_RANK_3: u64 = BB_A3 | BB_B3 | BB_C3 | BB_D3 | BB_E3 | BB_F3 | BB_G3 | BB_H3;
pub const BB_RANK_4: u64 = BB_A4 | BB_B4 | BB_C4 | BB_D4 | BB_E4 | BB_F4 | BB_G4 | BB_H4;
pub const BB_RANK_5: u64 = BB_A5 | BB_B5 | BB_C5 | BB_D5 | BB_E5 | BB_F5 | BB_G5 | BB_H5;
pub const BB_RANK_6: u64 = BB_A6 | BB_B6 | BB_C6 | BB_D6 | BB_E6 | BB_F6 | BB_G6 | BB_H6;
pub const BB_RANK_7: u64 = BB_A7 | BB_B7 | BB_C7 | BB_D7 | BB_E7 | BB_F7 | BB_G7 | BB_H7;
pub const BB_RANK_8: u64 = BB_A8 | BB_B8 | BB_C8 | BB_D8 | BB_E8 | BB_F8 | BB_G8 | BB_H8;

pub const BB_FILE_A: u64 = BB_A1 | BB_A2 | BB_A3 | BB_A4 | BB_A5 | BB_A6 | BB_A7 | BB_A8;
pub const BB_FILE_B: u64 = BB_B1 | BB_B2 | BB_B3 | BB_B4 | BB_B5 | BB_B6 | BB_B7 | BB_B8;
pub const BB_FILE_C: u64 = BB_C1 | BB_C2 | BB_C3 | BB_C4 | BB_C5 | BB_C6 | BB_C7 | BB_C8;
pub const BB_FILE_D: u64 = BB_D1 | BB_D2 | BB_D3 | BB_D4 | BB_D5 | BB_D6 | BB_D7 | BB_D8;
pub const BB_FILE_E: u64 = BB_E1 | BB_E2 | BB_E3 | BB_E4 | BB_E5 | BB_E6 | BB_E7 | BB_E8;
pub const BB_FILE_F: u64 = BB_F1 | BB_F2 | BB_F3 | BB_F4 | BB_F5 | BB_F6 | BB_F7 | BB_F8;
pub const BB_FILE_G: u64 = BB_G1 | BB_G2 | BB_G3 | BB_G4 | BB_G5 | BB_G6 | BB_G7 | BB_G8;
pub const BB_FILE_H: u64 = BB_H1 | BB_H2 | BB_H3 | BB_H4 | BB_H5 | BB_H6 | BB_H7 | BB_H8;

pub const BB_BACKRANKS: u64 = BB_RANK_1 | BB_RANK_8;

pub const BB_CORNERS: u64 = BB_A1 | BB_H1 | BB_A8 | BB_H8;
pub const BB_CENTER: u64 = BB_D4 | BB_E4 | BB_D5 | BB_E5;

pub const BB_NONE: u64 = 0;
pub const BB_ALL: u64 = 0xffff_ffff_ffff_ffff;
pub const BB_LIGHT_SQUARES: u64 = 0x55aa_55aa_55aa_55aa;
pub const BB_DARK_SQUARES: u64 = 0xaa55_aa55_aa55_aa55;

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
        self.ep_square = BB_NONE;
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
            ep_square: BB_NONE,
            occupied_co: [BB_RANK_1 | BB_RANK_2, BB_RANK_7 | BB_RANK_8],
        }
    }

    pub const fn clear() -> Self {
        Self {
            pawns: BB_NONE,
            knights: BB_NONE,
            bishops: BB_NONE,
            rooks: BB_NONE,
            queens: BB_NONE,
            kings: BB_NONE,
            castling_rights: BB_NONE,
            ep_square: BB_NONE,
            occupied_co: [BB_NONE, BB_NONE],
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
}