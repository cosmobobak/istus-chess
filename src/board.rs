#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

use std::fmt::{Debug, Display, Error, Formatter};
use crate::squares::SquareTrait;
use regex::Regex;

use crate::bitboards::{BB_A1, BB_A8, BB_C1, BB_C8, BB_D1, BB_D8, BB_E1, BB_E8, BB_F1, BB_F8, BB_G1, BB_G8, BB_H1, BB_H8, BB_RANK_1, BB_RANK_2, BB_RANK_4, BB_RANK_5, BB_RANK_7, BB_RANK_8, BB_ALL};

use crate::bitmethods::{Bithackable, into_bb};
use crate::bitboards::Bitboard;
use crate::cmove::{Move, MoveUndoInfo};
use crate::colour::Colour;
use crate::movebuffer::MoveBuf;
use crate::movegen::generate_pseudo_legal_moves;
use crate::piece::{Piece, Type};
use crate::squares::Square;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const STARTING_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

const SAN_REGEX_TEXT: &str =
    r#"^([NBKRQ])?([a-h])?([1-8])?[\-x]?([a-h][1-8])(=?[nbrqkNBRQK])?[\+#]?$"#;

lazy_static! {
    static ref SAN_REGEX: Regex = Regex::new(SAN_REGEX_TEXT).unwrap();
}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    bitboard: Bitboard,
    halfmove_clock: u8,
    fullmove_number: u16,
    moves_played: u16,
    stack: Vec<(Move, MoveUndoInfo)>,
}

impl Board {
    pub const fn new() -> Self {
        Self {
            bitboard: Bitboard::new(),
            halfmove_clock: 0,
            fullmove_number: 1,
            moves_played: 0,
            stack: Vec::new(),
        }
    }

    pub const fn clear() -> Self {
        Self {
            bitboard: Bitboard::clear(),
            halfmove_clock: 0,
            fullmove_number: 1,
            moves_played: 0,
            stack: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.bitboard.reset();
        self.moves_played = 0;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;
        self.stack.clear();
    }

    pub fn root(&self) -> Self {
        if self.stack.is_empty() {
            self.clone()
        } else {
            let mut root = self.clone();
            root.unplay_all();
            root
        }
    }

    fn unplay_all(&mut self) {
        while !self.stack.is_empty() {
            self.unmake();
        }
    }

    pub fn make(&mut self, m: Move) {
        use Type::{King, Pawn, Rook};

        let undo_info = MoveUndoInfo::new(
            self.bitboard.ep_square, 
            self.bitboard.castling_rights,
            self.halfmove_clock);

        let from = m.from_sq();
        let to = m.to_sq();
        let from_bb = into_bb(from as usize);
        let to_bb = into_bb(to as usize);
        let from_to_bb = from_bb | to_bb;
        let piece = self.bitboard.piece_type_at(from as usize).unwrap();
        let captured = self.bitboard.piece_type_at(to as usize);

        // clear the from_square and set the to_square in the colour bb
        self.bitboard.occupied_co[self.turn_as_idx()] ^= from_to_bb;

        // clear the from_square and set the to_square in the piece bb
        let bb = self.get_bb_mut(piece);
        *bb ^= from_to_bb;

        if let Some(captured) = captured {
            // clear the captured piece in the colour bb
            self.bitboard.occupied_co[1 - self.turn_as_idx()] ^= to_bb;
            let bb = self.get_bb_mut(captured);
            // clear the piece_bb
            *bb ^= to_bb;
        }
        
        // castling
        if piece == King {            
            if from_bb == BB_E1 && to_bb == BB_G1 {
                self.bitboard.rooks ^= BB_H1 | BB_F1;
            } else if from_bb == BB_E1 && to_bb == BB_C1 {
                self.bitboard.rooks ^= BB_A1 | BB_D1;
            } else if from_bb == BB_E8 && to_bb == BB_G8 {
                self.bitboard.rooks ^= BB_H8 | BB_F8;
            } else if from_bb == BB_E8 && to_bb == BB_C8 {
                self.bitboard.rooks ^= BB_A8 | BB_D8;
            }
        }

        // castling rights removal
        let castling_rights_mask = if self.bitboard.castling_rights.any_set() {
            if piece == King {
                if self.turn() == Colour::White {
                    BB_RANK_1
                } else {
                    BB_RANK_8
                }
            } else if piece == Rook {
                if from_bb == BB_A1 {
                    BB_A1
                } else if from_bb == BB_H1 {
                    BB_H1
                } else if from_bb == BB_A8 {
                    BB_A8
                } else if from_bb == BB_H8 {
                    BB_H8
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };

        self.bitboard.castling_rights &= !castling_rights_mask;

        // en passant capture
        if piece == Pawn && (to_bb & self.bitboard.ep_square).any_set() {
            let captured_pawn_loc = match self.turn() {
                Colour::Black => to_bb << 8,
                Colour::White => to_bb >> 8, 
            };
            // remove the captured pawn
            self.bitboard.pawns ^= captured_pawn_loc;
            // remove from the colour mask
            self.bitboard.occupied_co[1 - self.turn_as_idx()] ^= captured_pawn_loc;
        }

        // en passant square generation / removal
        self.bitboard.ep_square = 0;
        if piece == Pawn {
            if (from_bb & BB_RANK_2).any_set() && (to_bb & BB_RANK_4).any_set() {
                self.bitboard.ep_square = to_bb >> 8;
            } else if (from_bb & BB_RANK_7).any_set() && (to_bb & BB_RANK_5).any_set() {
                self.bitboard.ep_square = to_bb << 8;
            }
        }

        // promotions
        if let Some(promotion_piece_type) = m.promotion() {
            let promo_bb = match promotion_piece_type {
                Type::Knight => &mut self.bitboard.knights,
                Type::Bishop => &mut self.bitboard.bishops,
                Type::Rook => &mut self.bitboard.rooks,
                Type::Queen => &mut self.bitboard.queens,
                _ => unreachable!(),
            };
            *promo_bb |= to_bb;
            self.bitboard.pawns ^= to_bb;
        }

        // halfmove clock
        if piece == Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // fullmove number
        if self.turn() == Colour::White {
            self.fullmove_number += 1;
        }

        // move count
        self.moves_played += 1;

        // push the move and info onto the stack
        self.stack.push((m, undo_info));
    }

    pub fn unmake(&mut self) {
        let (last_move, info) = self.stack.pop().unwrap();
        self.unmake_unchecked(last_move, info);
    }

    fn unmake_unchecked(&mut self, last_move: Move, info: MoveUndoInfo) {
        use Type::{Bishop, King, Knight, Pawn, Queen, Rook};

        let MoveUndoInfo {
            ep_square: old_ep_square,
            castling_rights: old_castling_rights,
            halfmove_clock: old_halfmove_clock,
        } = info;

        let from = last_move.from_sq();
        let to = last_move.to_sq();
        let from_bb = into_bb(from as usize);
        let to_bb = into_bb(to as usize);
        let from_to_bb = from_bb | to_bb;
        let piece = self.bitboard.piece_type_at(to as usize).unwrap();
        let captured = last_move.capture();

        // promotions
        if let Some(promotion_piece_type) = last_move.promotion() {
            // determine the piece type to remove
            let promo_bb = match promotion_piece_type {
                Knight => &mut self.bitboard.knights,
                Bishop => &mut self.bitboard.bishops,
                Rook => &mut self.bitboard.rooks,
                Queen => &mut self.bitboard.queens,
                _ => unreachable!(),
            };
            *promo_bb ^= to_bb;
            // add the pawn back in (on the last rank, we move it later)
            self.bitboard.pawns ^= from_bb;
        }

        // clear the to_square and set the from_square in the piece bb
        // this cleans up pawns that appear on the last rank
        let bb = self.get_bb_mut(piece);
        *bb ^= from_to_bb;
        
        // clear the to_square and set the from_square in the colour bb
        self.bitboard.occupied_co[1 - self.turn_as_idx()] ^= from_to_bb;
        // if the move was a capture, then we reset the captured piece
        if let Some(captured_piece_type) = captured {
            // set the captured piece in the colour bb
            self.bitboard.occupied_co[self.turn_as_idx()] ^= to_bb;
            // set the captured piece in the piece bb
            let bb = self.get_bb_mut(captured_piece_type);
            *bb ^= to_bb;
        }

        // castling
        if piece == King {
            if from_bb == BB_E1 && to_bb == BB_G1 {
                self.bitboard.rooks ^= BB_H1 | BB_F1;
            } else if from_bb == BB_E1 && to_bb == BB_C1 {
                self.bitboard.rooks ^= BB_A1 | BB_D1;
            } else if from_bb == BB_E8 && to_bb == BB_G8 {
                self.bitboard.rooks ^= BB_H8 | BB_F8;
            } else if from_bb == BB_E8 && to_bb == BB_C8 {
                self.bitboard.rooks ^= BB_A8 | BB_D8;
            }
        }

        // en passant
        if piece == Pawn && (to_bb & old_ep_square).any_set() {
            let captured_pawn_loc = match self.turn() {
                Colour::White => old_ep_square << 8,
                Colour::Black => old_ep_square >> 8,
            };
            // add the captured pawn
            self.bitboard.pawns ^= captured_pawn_loc;
            // add to the colour mask
            self.bitboard.occupied_co[self.turn_as_idx()] ^= captured_pawn_loc;
        }

        // castling rights
        self.bitboard.castling_rights = old_castling_rights;

        // en passant square
        self.bitboard.ep_square = old_ep_square;

        // halfmove clock
        self.halfmove_clock = old_halfmove_clock;

        // fullmove number
        if self.turn() == Colour::Black {
            self.fullmove_number -= 1;
        }

        // move count
        self.moves_played -= 1;
    }

    fn get_bb_mut(&mut self, p: Type) -> &mut u64 {
        match p {
            Type::Pawn => &mut self.bitboard.pawns,
            Type::Knight => &mut self.bitboard.knights,
            Type::Bishop => &mut self.bitboard.bishops,
            Type::Rook => &mut self.bitboard.rooks,
            Type::Queen => &mut self.bitboard.queens,
            Type::King => &mut self.bitboard.kings,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Self::clear();
        board.set_from_fen(fen)?;
        Ok(board)
    }

    fn set_from_fen(&mut self, fen: &str) -> Result<(), String> {
        let mut parts = fen.split(' ').rev().collect::<Vec<_>>();
        let board_part = parts.pop().ok_or("empty fen")?;
        let turn_part = parts.pop().map_or(Ok(Colour::White), |tp| {
            match tp {
                "w" => Ok(Colour::White),
                "b" => Ok(Colour::Black),
                _ => Err(format!("expected 'w' or 'b' for turn part of fen: {}", fen)),
            }
        })?;
        let castling_part = parts.pop().map_or(Ok("-"), |cp| {
            // I refuse to learn how regex works.
            match cp {
                "-" | "K" | "Q" | "k" | "q" | "KQ" | "kq" | "Kk" | "Qq" | "Kq" | "Qk" | "KQk" | "KQq" | "Kkq" | "Qkq" | "KQkq" => Ok(cp),
                _ => Err(format!("invalid castling part in fen: {}", fen)),
            }
        })?;
        let ep_part = parts.pop().map_or(Ok(None), |ep| {
            if ep == "-" { Ok(None) } else {
                let ep = ep.as_bytes();
                if ep.len() != 2 {
                    return Err(format!("invalid ep part in fen: {}", fen));
                }
                let file = i32::from(ep[0]) - i32::from(b'a');
                let rank = i32::from(ep[1]) - i32::from(b'1');
                if !(0..8).contains(&file) || !(0..8).contains(&rank) {
                    return Err(format!("invalid ep part in fen: {}", fen));
                }
                let file = file as usize;
                let rank = rank as usize;
                Ok(Some(Square::from_rank_file(rank, file) as usize))
            }
        })?;
        let halfmove_part = parts.pop().map_or(Ok(0), |hmp| {
            hmp.parse::<usize>().map_err(|_| format!("invalid halfmove part in fen: {}", fen))
        })?;
        let fullmove_part = parts.pop().map_or(Ok(1), |fmp| {
            fmp.parse::<usize>().map_err(|_| format!("invalid fullmove part in fen: {}", fen)).map(|fmp| std::cmp::max(1, fmp))
        })?;
        if !parts.is_empty() {
            return Err(format!("fen string has more parts than expected: {}", fen));
        }

        // Validate the board part and set it.
        self.set_board_fen(board_part)?;

        // Apply.
        self.set_castling_fen(castling_part);
        self.bitboard.ep_square = ep_part.map_or(0, into_bb);
        self.halfmove_clock = halfmove_part as u8;
        self.fullmove_number = fullmove_part as u16;
        self.stack.clear();
        self.moves_played = (fullmove_part as u16 - 1) * 2 + (turn_part == Colour::Black) as u16;
        
        Ok(())
    }

    fn set_board_fen(&mut self, fen: &str) -> Result<(), String> {
        let fen = fen.trim();
        
        if fen.contains(' ') {
            return Err(format!("expected position part of fen, got multiple parts: {}", fen));
        }

        // Ensure the FEN is valid.
        let rows = fen.split('/').collect::<Vec<_>>();
        if rows.len() != 8 {
            return Err(format!("expected 8 rows in position part of fen, got {}: {}", rows.len(), fen));
        }

        // Validate each row.
        for &row in &rows {
            let mut field_sum = 0;
            let mut previous_was_digit = false;
            let mut previous_was_piece = false;

            for c in row.chars() {
                if ['1', '2', '3', '4', '5', '6', '7', '8'].contains(&c) {
                    if previous_was_digit {
                        return Err(format!("two subsequent digits in position part of fen: {}", fen));
                    }
                    field_sum += c as usize - '0' as usize;
                    previous_was_digit = true;
                    previous_was_piece = false;
                } else if c == '~' {
                    if !previous_was_piece {
                        return Err(format!("'~' not after piece in position part of fen: {}", fen));
                    }
                    previous_was_digit = false;
                    previous_was_piece = false;
                } else if ['p', 'n', 'b', 'r', 'q', 'k'].contains(&c.to_ascii_lowercase()) {
                    field_sum += 1;
                    previous_was_digit = false;
                    previous_was_piece = true;
                } else {
                    return Err(format!("invalid character in position part of fen: {}", fen));
                }
            }
            if field_sum != 8 {
                return Err(format!("expected 8 columns per row in position part of fen, got {}: {}", field_sum, fen));
            }
        }
        // Clear the board.
        // self._clear_board();

        // Put pieces on the board.
        let mut square_index = 0;
        for c in fen.chars() {
            if ['1', '2', '3', '4', '5', '6', '7', '8'].contains(&c) {
                square_index += c as usize - '0' as usize;
            } else if ['p', 'n', 'b', 'r', 'q', 'k'].contains(&c.to_ascii_lowercase()) {
                let piece = Piece::from_symbol(c)?;
                let square = square_index.flip_180();
                self.set_piece_at(square as usize, piece);
                square_index += 1;
            } else if c == '~' {
                todo!();
                // self.promoted |= into_bb(Square::from_index_180(square_index - 1) as usize);
            }
        }

        Ok(())
    }

    fn set_castling_fen(&mut self, castling_fen: &str) {
        if castling_fen.contains('K') {
            self.bitboard.castling_rights |= BB_H1;
        }
        if castling_fen.contains('Q') {
            self.bitboard.castling_rights |= BB_A1;
        }
        if castling_fen.contains('k') {
            self.bitboard.castling_rights |= BB_H8;
        }
        if castling_fen.contains('q') {
            self.bitboard.castling_rights |= BB_A8;
        }
    }

    fn board_fen(&self) -> String {
        let mut fen = String::new();
        let mut accumulator = 0;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = Square::from_rank_file(rank, file) as usize;
            
                if file == 0 {
                    if accumulator > 0 {
                        fen.push_str(&accumulator.to_string());
                        accumulator = 0;
                    }
                    if rank != 7 {
                        fen.push('/');
                    }
                }
                let p = self.get_piece_at(sq);
                if let Some(p) = p {
                    if accumulator > 0 {
                        fen.push_str(&accumulator.to_string());
                        accumulator = 0;
                    }
                    fen.push(p.symbol());
                } else {
                    accumulator += 1;
                }
            }
        }
        if accumulator > 0 {
            fen.push_str(&accumulator.to_string());
        }
        fen
    }

    fn epd(&self) -> String {
        let ep_square = if self.bitboard.ep_square.any_set() { 
            Some(self.bitboard.ep_square.lsb()) 
        } else { 
            None 
        };
        let turn_char = if self.turn() == Colour::White { "w" } else { "b" };
        let castling = if self.bitboard.castling_rights.any_set() { 
            format!("{}{}{}{}", 
                if (self.bitboard.castling_rights & BB_A1).any_set() { "K" } else { "" },
                if (self.bitboard.castling_rights & BB_H1).any_set() { "Q" } else { "" },
                if (self.bitboard.castling_rights & BB_A8).any_set() { "k" } else { "" },
                if (self.bitboard.castling_rights & BB_H8).any_set() { "q" } else { "" },
            )
        } else {
            "-".to_string()
        };
        let ep = ep_square.map_or_else(|| "-".to_string(), |ep_square| {
            format!("{}", ep_square) 
        });
        format!("{} {} {} {}", self.board_fen(), turn_char, castling, ep)
    }

    pub fn fen(&self) -> String {
        format!("{} {} {}", self.epd(), self.halfmove_clock, self.fullmove_number)
    }

    pub fn legal_moves(&self) -> MoveBuf {
        let mut buffer = MoveBuf::new();
        generate_pseudo_legal_moves(
            &mut buffer, 
            &self.bitboard, 
            self.turn_as_idx(), 
            BB_ALL, 
            BB_ALL);
        buffer
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        let square = square as usize;

        let piece_type = self.bitboard.piece_type_at(square)?;
        let colour = if self.bitboard.occupied_co[0].test(square) {
            Colour::White
        } else {
            Colour::Black
        };

        Some(Piece::new(piece_type, colour))
    }

    pub fn set_piece_at(&mut self, square: usize, piece: Piece) {
        assert!(square < 64);

        self.bitboard.occupied_co[piece.colour as usize].set(square);

        let piece_bb = match piece.piece_type {
            Type::Pawn => &mut self.bitboard.pawns,
            Type::Knight => &mut self.bitboard.knights,
            Type::Bishop => &mut self.bitboard.bishops,
            Type::Rook => &mut self.bitboard.rooks,
            Type::Queen => &mut self.bitboard.queens,
            Type::King => &mut self.bitboard.kings,
        };

        piece_bb.set(square);
    }

    pub fn turn(&self) -> Colour {
        self.moves_played.into()
    }

    pub const fn turn_as_idx(&self) -> usize {
        (self.moves_played & 1) as usize
    }

    pub fn parse_san(&self, move_san: &str) -> Result<Move, &str> {
        let move_san = move_san.trim();

        let side = self.turn_as_idx();
        if ["O-O", "O-O+", "O-O#", "0-0", "0-0+", "0-0#"].contains(&move_san) {
            // castling kingside.
            let kingloc = (self.bitboard.kings & self.bitboard.occupied_co[side]).lsb();
            let from = kingloc;
            let to = from + 2;
            return Ok(Move::new(from, to, None, None));
        } else if ["O-O-O", "O-O-O+", "O-O-O#", "0-0-0", "0-0-0+", "0-0-0#"].contains(&move_san) {
            // castling queenside.
            let kingloc = (self.bitboard.kings & self.bitboard.occupied_co[side]).lsb();
            let from = kingloc;
            let to = from - 2;
            return Ok(Move::new(from, to, None, None));
        }

        if !SAN_REGEX.is_match(move_san) {
            // Null moves.
            if ["--", "Z0", "0000", "@@@@"].contains(&move_san) {
                return Ok(Move::null());
            } else if move_san.contains(',') {
                return Err("unsupported multi-leg move");
            }
            return Err("invalid san");
        }

        todo!();
    }

    pub fn make_uci(&mut self, uci: &str) -> Result<(), &'static str> {
        let partial_move = Move::from_uci(uci)?;

        let capture = self.get_piece_at(partial_move.to_sq()).map(|p| p.piece_type);
        
        let actual_move = Move::new(
            partial_move.from_sq(),
            partial_move.to_sq(),
            capture,
            partial_move.promotion(),
        );
        self.make(actual_move);

        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut board = String::with_capacity(8 * 8 * 2 + 8); // ranks * files * 2 for each piece + 8 newlines.
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let piece = self.get_piece_at(square);
                if let Some(piece) = piece {
                    board.push(piece.symbol());
                } else {
                    board.push('.');
                }
                board.push(' ');
            }
            board.push('\n');
        }
        write!(f, "{}", board)?;
        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let bitboards = [
            self.bitboard.pawns,
            self.bitboard.knights,
            self.bitboard.bishops,
            self.bitboard.rooks,
            self.bitboard.queens,
            self.bitboard.kings,
            self.bitboard.occupied_co[0],
            self.bitboard.occupied_co[1],
            self.bitboard.castling_rights,
            self.bitboard.ep_square,
        ];
        let names = [
            "pawns", "knights", "bishops", "rooks", "queens", "kings", "white", "black", "castling rights", "en passant target square"
        ];
        for (name, bb) in names.iter().zip(bitboards.iter()) {
            writeln!(f, "bb: {}", name)?;
            for rank in (0..8).rev() {
                for file in 0..8 {
                    write!(f, "{} ", if bb.test(rank * 8 + file) { 'X' } else { '.' })?;
                }
                writeln!(f, " r{}", rank)?;
            }
        }
        writeln!(f, "turn: {:?}", self.turn())?;
        writeln!(f, "turn_as_idx: {}", self.turn_as_idx())?;
        writeln!(f, "moves played: {}", self.moves_played)?;
        Ok(())
    }
}

#[cfg(test)]
mod board_ops {
    use crate::board::Board;

    #[test]
    fn board_init_to_fen() {
        let b = Board::new();
        assert_eq!(b.fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn from_fen() {
        let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(b, Board::new());
    }

    #[test]
    fn fen_fuzzing() {
        let fens = include_str!("../puzzles.txt");
        let fens: Vec<&str> = fens.lines().collect();
        for (i, &fen) in fens.iter().enumerate() {
            let b = Board::from_fen(fen).unwrap_or_else(|err| {
                panic!("Failed to parse! \n    FEN: {} \n    at index {} \n    with error {}", fen, i, err)
            });
            assert_eq!(b.fen(), fen, "FAIL - FEN: {}", fen);
        }
    }
}

#[cfg(test)]
mod move_make {
    use crate::cmove::Move;
    use crate::colour::Colour;
    use crate::piece::{Type, Piece};
    use crate::squares::SquareEnum;
    use SquareEnum::{E2, E4, F5, G7, G8};
    use crate::board::Board;

    #[test]
    fn make_move() {
        let mut board = Board::new();
        board.make(Move::from_uci("e2e4").unwrap());
        assert_eq!(board.get_piece_at(E4 as usize).unwrap().piece_type, Type::Pawn);
        assert_eq!(board.get_piece_at(E2 as usize), None);
    }

    #[test]
    fn make_move_promo() {
        let ucis = vec!["h2h4", "g7g5", "h4g5", "g8h6", "g5h6", "f8g7", "h6g7", "a7a5"];
        let mut refboard = Board::new();
        for m in ucis {
            refboard.make_uci(m).unwrap();
        }
        // after playing all of the moves, the board is in position for white to promote the pawn on g7.
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8q").unwrap());
        assert_eq!(board.get_piece_at(G8 as usize), Some(Piece::new(Type::Queen, Colour::White)));
        assert_eq!(board.get_piece_at(G7 as usize), None);
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8n").unwrap());
        assert_eq!(board.get_piece_at(G8 as usize), Some(Piece::new(Type::Knight, Colour::White)));
        assert_eq!(board.get_piece_at(G7 as usize), None);
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8r").unwrap());
        assert_eq!(board.get_piece_at(G8 as usize), Some(Piece::new(Type::Rook, Colour::White)));
        assert_eq!(board.get_piece_at(G7 as usize), None);
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8b").unwrap());
        assert_eq!(board.get_piece_at(G8 as usize), Some(Piece::new(Type::Bishop, Colour::White)));
        assert_eq!(board.get_piece_at(G7 as usize), None);
    }

    #[test]
    fn make_unmake() {
        let moves = vec![
            "e2e4",
            "e7e5",
            "g1f3",
        ];
        let mut board = Board::new();
        for &m in &moves {
            println!("{}", board);
            board.make_uci(m).unwrap();
        }
        for _ in 0..moves.len() {
            println!("{}", board);
            board.unmake();
        }
        
        assert_eq!(board, Board::new());
    }

    #[test]
    fn make_unmake_complex() {
        let moves = vec![
            "e2e4",
            "e7e5",
            "g1f3",
            "b8c6",
            "f1c4",
            "d7d6",
            "e1g1",
            "d8h4",
            "f3h4",
        ];
        let mut board = Board::new();
        for &m in &moves {
            println!("{}", board);
            board.make_uci(m).unwrap();
        }
        for _ in 0..moves.len() {
            println!("{}", board);
            board.unmake();
        }
        
        assert_eq!(board, Board::new());
    }

    #[test]
    fn en_passant() {
        let moves = vec![
            "e2e4",
            "d7d5",
            "e4e5",
            "f7f5",
            "e5f6",
        ];
        let mut board = Board::new();
        for &m in &moves {
            println!("{}", board);
            board.make_uci(m).unwrap();
        }
        println!("{}", board);
        assert_eq!(board.get_piece_at(F5 as usize), None);
        board.unmake();
        println!("{}", board);
        assert_eq!(board.get_piece_at(F5 as usize), Some(Piece::new(Type::Pawn, Colour::Black)));
    }
}