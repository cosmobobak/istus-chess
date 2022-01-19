#![allow(clippy::cast_possible_wrap)]

pub type Square = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum SquareEnum {
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8
}

pub type Rank = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum RankEnum {
    R1, R2, R3, R4, R5, R6, R7, R8
}

pub type File = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum FileEnum {
    A, B, C, D, E, F, G, H
}

pub trait SquareTrait {
    fn rank(self) -> Rank;
    fn file(self) -> File;
    fn from_rank_file(rank: Rank, file: File) -> Square;
    fn flip_180(self) -> Square;
    fn square_distance(a: Square, b: Square) -> usize;
}

impl SquareTrait for Square {
    fn from_rank_file(rank: Rank, file: File) -> Self {
        rank * 8 + file
    }

    fn square_distance(a: Square, b: Square) -> usize {
        std::cmp::max(
            (a.rank() as isize - b.rank() as isize).abs() as Self,
            (a.file() as isize - b.file() as isize).abs() as Self
        )
    }

    fn flip_180(self) -> Self {
        self ^ 0x38
    }

    fn rank(self) -> Rank {
        self >> 3
    }

    fn file(self) -> File {
        self & 7
    }
}
