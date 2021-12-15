use crate::bitboards::{BB_FILE_A, BB_FILE_H, BB_FILE_G, BB_FILE_B};



pub trait Bithackable {
    fn test(self, n: usize) -> bool;
    fn has_any_set(self) -> bool;
    fn lsb(self) -> usize;
    fn msb(self) -> usize;
    fn popcount(self) -> usize;
    fn to_vec(self) -> Vec<usize>;
    fn iter_bits(self) -> IterBits;
    fn set(&mut self, n: usize);
    fn clear(&mut self, n: usize);
    fn flip(&mut self, n: usize);
    fn clear_lsb(&mut self);
    fn clear_msb(&mut self);
    fn show_debug(self);
    fn flip_vertical(self) -> Self;
    fn flip_horizontal(self) -> Self;
    fn flip_diagonal(self) -> Self;
    fn flip_anti_diagonal(self) -> Self;
    fn shift_down(self) -> Self;
    fn shift_2_down(self) -> Self;
    fn shift_up(self) -> Self;
    fn shift_2_up(self) -> Self;
    fn shift_right(self) -> Self;
    fn shift_2_right(self) -> Self;
    fn shift_left(self) -> Self;
    fn shift_2_left(self) -> Self;
    fn shift_up_left(self) -> Self;
    fn shift_up_right(self) -> Self;
    fn shift_down_left(self) -> Self;
    fn shift_down_right(self) -> Self;
}

impl Bithackable for u64 {
    fn test(self, n: usize) -> bool {
        debug_assert!(n < 64);
        (self & (1 << n)) != 0
    }

    fn has_any_set(self) -> bool {
        self != 0
    }

    fn set(&mut self, n: usize) {
        debug_assert!(n < 64);
        *self |= 1 << n;
    }

    fn clear(&mut self, n: usize) {
        debug_assert!(n < 64);
        *self &= !(1 << n);
    }

    fn flip(&mut self, n: usize) {
        debug_assert!(n < 64);
        *self ^= 1 << n;
    }

    fn lsb(self) -> usize {
        self.trailing_zeros() as usize
    }

    fn msb(self) -> usize {
        self.leading_zeros() as usize
    }

    fn popcount(self) -> usize {
        self.count_ones() as usize
    }

    fn clear_lsb(&mut self) {
        *self &= *self - 1;
    }

    fn clear_msb(&mut self) {
        *self &= !(1 << self.leading_zeros());
    }

    fn to_vec(self) -> Vec<usize> {
        self.iter_bits().collect()
    }

    fn iter_bits(self) -> IterBits {
        IterBits { bitboard: self }
    }

    fn show_debug(self) {
        for row in 0..8 {
            for col in 0..8 {
                if self.test(row * 8 + col) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn flip_vertical(self) -> Self {
        // https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#FlipVertically
        let mut bb = self;
        bb = ((bb >> 8) & 0x00ff_00ff_00ff_00ff) | ((bb & 0x00ff_00ff_00ff_00ff) << 8);
        bb = ((bb >> 16) & 0x0000_ffff_0000_ffff) | ((bb & 0x0000_ffff_0000_ffff) << 16);
        (bb >> 32) | ((bb & 0x0000_0000_ffff_ffff) << 32)
    }

    fn flip_horizontal(self) -> Self {
        // https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#MirrorHorizontally
        let mut bb = self;
        bb = ((bb >> 1) & 0x5555_5555_5555_5555) | ((bb & 0x5555_5555_5555_5555) << 1);
        bb = ((bb >> 2) & 0x3333_3333_3333_3333) | ((bb & 0x3333_3333_3333_3333) << 2);
        ((bb >> 4) & 0x0f0f_0f0f_0f0f_0f0f) | ((bb & 0x0f0f_0f0f_0f0f_0f0f) << 4)
    }

    fn flip_diagonal(self) -> Self {
        // https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#FlipabouttheDiagonal
        let mut bb = self;
        let mut t = (bb ^ (bb << 28)) & 0x0f0f_0f0f_0000_0000;
        bb = bb ^ t ^ (t >> 28);
        t = (bb ^ (bb << 14)) & 0x3333_0000_3333_0000;
        bb = bb ^ t ^ (t >> 14);
        t = (bb ^ (bb << 7)) & 0x5500_5500_5500_5500;
        bb ^ t ^ (t >> 7)
    }

    fn flip_anti_diagonal(self) -> Self {
        // https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#FlipabouttheAntidiagonal
        let mut bb = self;
        let mut t = bb ^ (bb << 36);
        bb = bb ^ ((t ^ (bb >> 36)) & 0xf0f0_f0f0_0f0f_0f0f);
        t = (bb ^ (bb << 18)) & 0xcccc_0000_cccc_0000;
        bb = bb ^ t ^ (t >> 18);
        t = (bb ^ (bb << 9)) & 0xaa00_aa00_aa00_aa00;
        bb ^ t ^ (t >> 9)
    }

    fn shift_down(self) -> Self {
        self >> 8
    }

    fn shift_2_down(self) -> Self {
        self >> 16
    }

    fn shift_up(self) -> Self {
        self << 8
    }

    fn shift_2_up(self) -> Self {
        self << 16
    }

    fn shift_right(self) -> Self {
        (self << 1) & !BB_FILE_A
    }

    fn shift_2_right(self) -> Self {
        (self << 2) & !BB_FILE_A & !BB_FILE_B
    }

    fn shift_left(self) -> Self {
        (self >> 1) & !BB_FILE_H
    }

    fn shift_2_left(self) -> Self {
        (self >> 2) & !BB_FILE_G & !BB_FILE_H
    }

    fn shift_up_left(self) -> Self {
        (self << 7) & !BB_FILE_H
    }

    fn shift_up_right(self) -> Self {
        (self << 9) & !BB_FILE_A
    }

    fn shift_down_left(self) -> Self {
        (self >> 9) & !BB_FILE_H
    }

    fn shift_down_right(self) -> Self {
        (self >> 7) & !BB_FILE_A
    }
}

pub const fn into_bb(n: usize) -> u64 {
    1 << n
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IterBits {
    bitboard: u64,
}

impl Iterator for IterBits {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.bitboard == 0 {
            None
        } else {
            let bit = self.bitboard.lsb();
            // clear_lsb() is likely faster than clear(bit) as it's (subtract->and) rather than (shift->not->and)
            self.bitboard.clear_lsb(); 
            // if bit == 0 {
            //     unsafe { std::hint::unreachable_unchecked(); }
            // }
            Some(bit)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.bitboard.popcount();
        (count, Some(count))
    }
}