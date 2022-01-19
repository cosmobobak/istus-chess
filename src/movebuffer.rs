use std::fmt::{Formatter, Error};
use std::fmt::Debug;
use std::ops::Index;
use crate::cmove::Move;

pub struct MoveBuf {
    len: usize,
    buffer: [Move; 256],
}

impl Debug for MoveBuf {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "MoveBuf {:?}", &self.buffer[..self.len])
    }
}

impl MoveBuf {
    pub const fn new() -> Self {
        Self {
            buffer: [Move::null(); 256],
            len: 0,
        }
    }

    pub fn push(&mut self, m: Move) {
        assert!(self.len < 256);
        // SAFETY: We have just done a bounds check, 
        // so we are safe to write to the buffer
        unsafe { *self.buffer.get_unchecked_mut(self.len) = m; }
        self.len += 1;
    }

    pub const fn len(&self) -> usize {
        self.len
    }
}

impl Index<usize> for MoveBuf {
    type Output = Move;

    fn index(&self, index: usize) -> &Move {
        assert!(index < self.len, "Index out of bounds");
        // SAFETY: We have just done a bounds check, 
        // so we are safe to read from the buffer
        unsafe { self.buffer.get_unchecked(index) }
    }
}

impl<'a> IntoIterator for &'a MoveBuf {
    type Item = &'a Move;
    type IntoIter = std::slice::Iter<'a, Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer[..self.len].iter()
    }
}