use crate::colour::Colour;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: Type,
    pub colour: Colour,
}

impl Piece {
    pub const fn new(piece_type: Type, colour: Colour) -> Self {
        Self { piece_type, colour }
    }

    pub const fn symbol(self) -> char {
        // white pieces are capitalised, black are lowercase
        let byte_offset = match self.colour {
            Colour::Black => b'a' - b'A',
            Colour::White => 0,
        };
        (match self.piece_type {
            Type::Pawn => b'P' + byte_offset,
            Type::Knight => b'N' + byte_offset,
            Type::Bishop => b'B' + byte_offset,
            Type::Rook => b'R' + byte_offset,
            Type::Queen => b'Q' + byte_offset,
            Type::King => b'K' + byte_offset,
        }) as char
    }

    pub fn from_symbol(symbol: char) -> Result<Self, &'static str> {
        const VALID_CHARS: &str = "PNBRQKpnbrqk";
        if !VALID_CHARS.contains(symbol) {
            return Err("Invalid piece symbol");
        }
        let colour = if symbol.is_ascii_uppercase() { 
            Colour::White 
        } else { 
            Colour::Black 
        };
        let piece_type = match symbol {
            'P' | 'p' => Type::Pawn,
            'N' | 'n' => Type::Knight,
            'B' | 'b' => Type::Bishop,
            'R' | 'r' => Type::Rook,
            'Q' | 'q' => Type::Queen,
            'K' | 'k' => Type::King,
            _ => unreachable!(),
        };
        Ok(Self {
            piece_type,
            colour,
        })
    }
}
