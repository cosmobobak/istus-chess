use crate::colour::Colour;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
}

impl Piece {
    pub const fn new(piece_type: PieceType, colour: Colour) -> Self {
        Self { piece_type, colour }
    }

    pub const fn symbol(self) -> char {
        // white pieces are capitalised, black are lowercase
        let byte_offset = match self.colour {
            Colour::Black => b'a' - b'A',
            Colour::White => 0,
        };
        (match self.piece_type {
            PieceType::Pawn => b'P' + byte_offset,
            PieceType::Knight => b'N' + byte_offset,
            PieceType::Bishop => b'B' + byte_offset,
            PieceType::Rook => b'R' + byte_offset,
            PieceType::Queen => b'Q' + byte_offset,
            PieceType::King => b'K' + byte_offset,
            PieceType::None => b'?',
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
            'P' | 'p' => PieceType::Pawn,
            'N' | 'n' => PieceType::Knight,
            'B' | 'b' => PieceType::Bishop,
            'R' | 'r' => PieceType::Rook,
            'Q' | 'q' => PieceType::Queen,
            'K' | 'k' => PieceType::King,
            _ => unreachable!(),
        };
        Ok(Self {
            piece_type,
            colour,
        })
    }
}
