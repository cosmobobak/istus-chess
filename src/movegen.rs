use crate::{bitboards::{BB_FILE_A, BB_FILE_H, Bitboard, BB_ALL}, bitmethods::Bithackable, cmove::Move, colour::Colour, piece::Type};

use crate::magicnumbers::{BB_KING_ATTACKS, BB_KNIGHT_ATTACKS, BB_PAWN_ATTACKS, BB_RANK_ATTACKS, BB_FILE_ATTACKS, BB_DIAG_ATTACKS, BB_RAYS};

use crate::bitmethods::into_bb;

fn ray(a: usize, b: usize) -> u64 {
    BB_RAYS[a][b]
}

fn between(a: usize, b: usize) -> u64 {
    let bb = BB_RAYS[a][b] & ((BB_ALL << a) ^ (BB_ALL << b));
    bb & (bb - 1)
}

fn slider_blockers(state: &Bitboard, origin: usize, turn: Colour) -> u64 {
    let occupied = state.occupied_co[0] | state.occupied_co[1];
    let rooks_and_queens = state.rooks | state.queens;
    let bishops_and_queens = state.bishops | state.queens;

    let snipers = (BB_RANK_ATTACKS[origin] & rooks_and_queens) |
                   (BB_FILE_ATTACKS[origin] & rooks_and_queens) |
                   (BB_DIAG_ATTACKS[origin] & bishops_and_queens);

    let mut blockers = 0;

    for sniper in snipers.iter_bits() {
        let b = between(origin, sniper) & occupied;
        // Add to blockers if exactly one piece in-between.
        if b.has_any_set() && into_bb(b.msb()) == b {
            blockers |= b;
        }
    }

    blockers & state.occupied_co[turn as usize]
}

fn generate_evasions(buffer: &mut Vec<Move>, state: &Bitboard, turn: Colour, king: usize, checkers: u64, from_mask: u64, to_mask: u64) {
    let sliders = checkers & (state.bishops | state.rooks | state.queens);

    let mut attacked = 0;
    for checker in sliders.iter_bits() {
        attacked |= ray(king, checker) & !into_bb(checker);
    }

    let bb = BB_KING_ATTACKS[king] & !state.occupied_co[turn as usize] & !attacked;
    for to_square in bb.iter_bits() {
        let capture = state.piece_type_at(to_square);
        buffer.push(Move::new(king.into(), to_square.into(), capture, None));
    }
    

    let checker = checkers.lsb();
    if into_bb(checker) == checkers {
        // Capture or block a single checker.
        let target = between(king, checker) | checkers;

        generate_pseudo_legal_moves(buffer, state, turn, !state.kings & from_mask, target & to_mask);

        // Capture the checking pawn en passant (but avoid yielding
        //  duplicate moves).
        if state.ep_square.has_any_set() && !(state.ep_square & target).has_any_set() {
            let last_double = if turn == Colour::White {state.ep_square.lsb() - 8} else {state.ep_square.lsb() + 8};
            if last_double == checker {
                generate_pseudo_legal_ep(buffer, state, turn, from_mask, to_mask);
            }
        }
    }
}

fn generate_pseudo_legal_ep(buffer: &mut Vec<Move>, state: &Bitboard, turn: Colour, from_mask: u64, to_mask: u64) {
    todo!();
}

pub(crate) fn generate_pseudo_legal_moves(buffer: &mut Vec<Move>, state: &Bitboard, turn: Colour, from_mask: u64, to_mask: u64) {
    // This function fills a vector with all pseudo legal moves.
    // IDEAS FOR OPTIMISATION:
    // - special case the get_piece_at function, as we never capture kings, and always capture Something
    // - order captures first (we will sort the moves later, but this probably helps anyway)

    let our_pieces = state.occupied_co[turn as usize];
    let their_pieces = state.occupied_co[1 - turn as usize];
    let our_pawns = state.pawns & our_pieces;
    let pawn_offset: isize = match turn {
        Colour::White => 8,
        Colour::Black => -8,
    };
    // pawn pushes.
    for pawn_loc in our_pawns.iter_bits() {
        // single pawn moves
        let target_square = (pawn_loc as isize + pawn_offset) as usize;
        let is_blocked = (into_bb(target_square) & their_pieces).has_any_set();
        if !is_blocked {
            if target_square >= 56 || target_square <= 7 {
                for &pt in &[Type::Queen, Type::Knight, Type::Rook, Type::Bishop] {
                    buffer.push(Move::new(
                        pawn_loc.into(), 
                        target_square.into(), 
                        None,
                        Some(pt)));
                }
            } else {
                buffer.push(Move::new(
                        pawn_loc.into(), 
                        target_square.into(), 
                        None,
                        None));
            }
        }
        // double pawn moves
        let target_square = (pawn_loc as isize + 2 * pawn_offset) as usize;
        let is_blocked = (into_bb(target_square) & their_pieces).has_any_set();
        let can_double_move = match turn {
            Colour::White => pawn_loc > 7 && pawn_loc < 16,
            Colour::Black => pawn_loc > 48 && pawn_loc < 56,
        };
        if !is_blocked && can_double_move {
            buffer.push(Move::new(
                pawn_loc.into(), 
                target_square.into(), 
            None, None));
        }
    }

    // pawn captures.
    for pawn_loc in our_pawns.iter_bits() {
        let moves_bb = BB_PAWN_ATTACKS[turn as usize][pawn_loc as usize];
        let attacks_bb = moves_bb & their_pieces;
        for target_square in attacks_bb.iter_bits() {
            // can optimise the call to piece_type_at later.
            let target_piece_type = state.piece_type_at(target_square);
            if target_square >= 56 || target_square <= 7 {
                for &pt in &[Type::Queen, Type::Knight, Type::Rook, Type::Bishop] {
                    buffer.push(Move::new(
                        pawn_loc.into(), 
                        target_square.into(), 
                        target_piece_type,
                    Some(pt)));
                }
            } else {
                buffer.push(Move::new(
                    pawn_loc.into(), 
                    target_square.into(), 
                    target_piece_type,
                    None));
            }
        }

        // en passant captures
        let ep_attacks_bb = moves_bb & state.ep_square;
        if ep_attacks_bb.has_any_set() {
            let target_square = ep_attacks_bb.lsb();
            buffer.push(Move::new(
                pawn_loc.into(), 
                target_square.into(), 
                Some(Type::Pawn),
                None));
        }
    }

    // knight moves and captures.
    let knight_locs = our_pieces & state.knights;
    for knight_loc in knight_locs.iter_bits() {
        let moves_bb = BB_KNIGHT_ATTACKS[knight_loc];
        let quiet_bb = moves_bb & !(their_pieces | our_pieces);
        let captures_bb = moves_bb & their_pieces;
        for target_square in quiet_bb.iter_bits() {
            buffer.push(Move::new(
                knight_loc.into(), 
                target_square.into(), 
                None,
                None));
        }
        for target_square in captures_bb.iter_bits() {
            let target_piece_type = state.piece_type_at(target_square);
            buffer.push(Move::new(
                knight_loc.into(), 
                target_square.into(), 
                target_piece_type,
                None));
        }
    }

    // king moves and captures.
    let king_loc = (our_pieces & state.kings).lsb();
    let kmoves_bb = BB_KING_ATTACKS[king_loc];
    let kquiet_bb = kmoves_bb & !(their_pieces | our_pieces);
    let kcaptures_bb = kmoves_bb & their_pieces;
    for target_square in kquiet_bb.iter_bits() {
        buffer.push(Move::new(
            king_loc.into(), 
            target_square.into(), 
            None,
            None));
    }
    for target_square in kcaptures_bb.iter_bits() {
        let target_piece_type = state.piece_type_at(target_square);
        buffer.push(Move::new(
            king_loc.into(), 
            target_square.into(), 
            target_piece_type,
            None));
    }

    // castling moves.
    let can_castle_kingside = (state.castling_rights & BB_FILE_H).has_any_set();
    let can_castle_queenside = (state.castling_rights & BB_FILE_A).has_any_set();

    if can_castle_kingside {
        let target_square = king_loc + 2;
        buffer.push(Move::new(
            king_loc.into(), 
            target_square.into(), 
            None,
            None));
    }
    if can_castle_queenside {
        let target_square = king_loc - 2;
        buffer.push(Move::new(
            king_loc.into(), 
            target_square.into(), 
            None,
            None));
    }
}

pub fn generate_legal_moves(buffer: &mut Vec<Move>, state: &Bitboard, turn: Colour) {
    todo!();
}