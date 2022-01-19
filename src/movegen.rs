use crate::{
    bitboards::Bitboard,
    bitmethods::Bithackable,
    cmove::Move,
    colour::{BLACK, WHITE},
    magicnumbers::{
        BB_ALL, BB_DIAG_MASKS, BB_EMPTY, BB_FILE_C, BB_FILE_D, BB_FILE_E, BB_FILE_F, BB_FILE_G,
        BB_FILE_MASKS, BB_RANKS, BB_RANK_1, BB_RANK_3, BB_RANK_4, BB_RANK_5, BB_RANK_6, BB_RANK_8,
        BB_RANK_MASKS,
    },
    piece::Type,
    squares::{Square, SquareTrait},
};

use crate::magicnumbers::{
    BB_DIAG_ATTACKS, BB_FILE_ATTACKS, BB_KING_ATTACKS, BB_KNIGHT_ATTACKS, BB_PAWN_ATTACKS,
    BB_RANK_ATTACKS, BB_RAYS,
};

use crate::bitmethods::into_bb;
use crate::movebuffer::MoveBuf;

fn ray(a: usize, b: usize) -> u64 {
    BB_RAYS[a][b]
}

fn between(a: usize, b: usize) -> u64 {
    let bb = BB_RAYS[a][b] & ((BB_ALL << a) ^ (BB_ALL << b));
    bb & (bb - 1)
}

// fn slider_blockers(state: &Bitboard, origin: usize, turn: Colour) -> u64 {
//     let occupied = state.occupied_co[0] | state.occupied_co[1];
//     let rooks_and_queens = state.rooks | state.queens;
//     let bishops_and_queens = state.bishops | state.queens;

//     let snipers = (BB_RANK_ATTACKS[origin] & rooks_and_queens)
//         | (BB_FILE_ATTACKS[origin] & rooks_and_queens)
//         | (BB_DIAG_ATTACKS[origin] & bishops_and_queens);

//     let mut blockers = 0;

//     for sniper in snipers.iter_bits() {
//         let b = between(origin, sniper) & occupied;
//         // Add to blockers if exactly one piece in-between.
//         if b.any_set() && into_bb(b.msb()) == b {
//             blockers |= b;
//         }
//     }

//     blockers & state.occupied_co[turn as usize]
// }

fn attackers_mask(state: &Bitboard, turn_idx: usize, square: usize, occupied: u64) -> u64 {
    let rank_pieces = BB_RANK_MASKS[square] & occupied;
    let file_pieces = BB_FILE_MASKS[square] & occupied;
    let diag_pieces = BB_DIAG_MASKS[square] & occupied;

    let queens_and_rooks = state.queens | state.rooks;
    let queens_and_bishops = state.queens | state.bishops;

    let attackers = (BB_KING_ATTACKS[square] & state.kings)
        | (BB_KNIGHT_ATTACKS[square] & state.knights)
        | (BB_RANK_ATTACKS[square][&rank_pieces] & queens_and_rooks)
        | (BB_FILE_ATTACKS[square][&file_pieces] & queens_and_rooks)
        | (BB_DIAG_ATTACKS[square][&diag_pieces] & queens_and_bishops)
        | (BB_PAWN_ATTACKS[1 ^ turn_idx][square] & state.pawns);

    attackers & state.occupied_co[turn_idx]
}

fn attacked_for_king(state: &Bitboard, turn_idx: usize, path: u64, occupied: u64) -> bool {
    path.iter_bits()
        .any(|sq| attackers_mask(state, turn_idx ^ 1, sq, occupied) != 0)
}

fn generate_castling_moves(
    buffer: &mut MoveBuf,
    state: &Bitboard,
    turn_idx: usize,
    from_mask: u64,
    to_mask: u64,
) {
    let backrank = if turn_idx == WHITE {
        BB_RANK_1
    } else {
        BB_RANK_8
    };
    let king_bb = state.occupied_co[turn_idx] & state.kings & backrank & from_mask & BB_FILE_E;
    if king_bb.popcount() != 1 {
        return;
    }
    let king_sq = king_bb.lsb();
    let our_rights = state.castling_rights & backrank & to_mask;

    let bb_c = BB_FILE_C & backrank;
    let bb_d = BB_FILE_D & backrank;
    let bb_f = BB_FILE_F & backrank;
    let bb_g = BB_FILE_G & backrank;

    for candidate in our_rights.iter_bits() {
        let rook_bb = into_bb(candidate);

        let a_side = rook_bb < king_bb;
        let king_to = if a_side { bb_c } else { bb_g };
        let rook_to = if a_side { bb_d } else { bb_f };

        let king_path = between(king_bb.lsb(), king_to.lsb());
        let rook_path = between(candidate, rook_to.lsb());

        if !(((state.occupied() ^ king_bb ^ rook_bb) & (king_path | rook_path | king_to | rook_to))
            .any_set()
            || attacked_for_king(
                state,
                turn_idx,
                king_path | king_bb,
                state.occupied() ^ king_bb,
            )
            || attacked_for_king(
                state,
                turn_idx,
                king_to,
                state.occupied() ^ king_bb ^ rook_bb ^ rook_to,
            ))
        {
            buffer.push(Move::new(king_sq, king_to.lsb(), None, None));
        }
    }
}

fn generate_evasions(
    buffer: &mut MoveBuf,
    state: &Bitboard,
    turn_idx: usize,
    king: usize,
    checkers: u64,
    from_mask: u64,
    to_mask: u64,
) {
    let sliders = checkers & (state.bishops | state.rooks | state.queens);

    let mut attacked = 0;
    for checker in sliders.iter_bits() {
        attacked |= ray(king, checker) & !into_bb(checker);
    }

    let bb = BB_KING_ATTACKS[king] & !state.occupied_co[turn_idx] & !attacked;
    for to_square in bb.iter_bits() {
        let capture = state.piece_type_at(to_square);
        buffer.push(Move::new(king, to_square, capture, None));
    }

    let checker = checkers.lsb();
    if into_bb(checker) == checkers {
        // Capture or block a single checker.
        let target = between(king, checker) | checkers;

        generate_pseudo_legal_moves(
            buffer,
            state,
            turn_idx,
            !state.kings & from_mask,
            target & to_mask,
        );

        // Capture the checking pawn en passant (but avoid yielding
        //  duplicate moves).
        if state.ep_square.any_set() && !(state.ep_square & target).any_set() {
            let last_double = if turn_idx == WHITE {
                state.ep_square.lsb() - 8
            } else {
                state.ep_square.lsb() + 8
            };
            if last_double == checker {
                generate_pseudo_legal_ep(buffer, state, turn_idx, from_mask, to_mask);
            }
        }
    }
}

pub fn generate_pseudo_legal_moves(
    buffer: &mut MoveBuf,
    state: &Bitboard,
    turn_idx: usize,
    from_mask: u64,
    to_mask: u64,
) {
    // This function fills a vector with all pseudo legal moves.
    // IDEAS FOR OPTIMISATION:
    // - special case the get_piece_at function, as we never capture kings, and always capture Something
    // - order captures first (we will sort the moves later, but this probably helps anyway)

    let our_pieces = state.occupied_co[turn_idx];

    // Generate piece moves.
    let non_pawns = our_pieces & !state.pawns & from_mask;
    for from_square in non_pawns.iter_bits() {
        let moves = attacks_mask(state, from_square) & !our_pieces & to_mask;
        for to_square in moves.iter_bits() {
            let capture = state.piece_type_at(to_square);
            buffer.push(Move::new(from_square, to_square, capture, None));
        }
    }

    // Generate castling moves.
    if from_mask & state.kings != 0 {
        generate_castling_moves(buffer, state, turn_idx, from_mask, to_mask);
    }

    // The remaining moves are all pawn moves.
    let pawns = state.pawns & state.occupied_co[turn_idx] & from_mask;
    if !pawns.any_set() {
        return;
    }

    // Generate pawn captures.
    let capturers = pawns;
    for from_square in capturers.iter_bits() {
        let targets =
            BB_PAWN_ATTACKS[turn_idx][from_square] & state.occupied_co[1 ^ turn_idx] & to_mask;

        for to_square in targets.iter_bits() {
            let to_square: Square = to_square;
            let capture = state.piece_type_at(to_square);
            if to_square.rank() == 0 || to_square.rank() == 7 {
                buffer.push(Move::new(
                    from_square,
                    to_square,
                    capture,
                    Some(Type::Queen),
                ));
                buffer.push(Move::new(
                    from_square,
                    to_square,
                    capture,
                    Some(Type::Knight),
                ));
                buffer.push(Move::new(from_square, to_square, capture, Some(Type::Rook)));
                buffer.push(Move::new(
                    from_square,
                    to_square,
                    capture,
                    Some(Type::Bishop),
                ));
            } else {
                buffer.push(Move::new(from_square, to_square, capture, None));
            }
        }
    }

    // Prepare pawn advance generation.

    let mut single_moves;
    let mut double_moves;
    if turn_idx == WHITE {
        single_moves = pawns << 8 & !state.occupied();
        double_moves = single_moves << 8 & !state.occupied() & (BB_RANK_3 | BB_RANK_4);
    } else {
        single_moves = pawns >> 8 & !state.occupied();
        double_moves = single_moves >> 8 & !state.occupied() & (BB_RANK_6 | BB_RANK_5);
    };

    single_moves &= to_mask;
    double_moves &= to_mask;

    // Generate single pawn moves.
    for to_square in single_moves.iter_bits() {
        let from_square = if turn_idx == BLACK {
            to_square + 8
        } else {
            to_square - 8
        };

        if to_square.rank() == 0 || to_square.rank() == 7 {
            buffer.push(Move::new(from_square, to_square, None, Some(Type::Queen)));
            buffer.push(Move::new(from_square, to_square, None, Some(Type::Knight)));
            buffer.push(Move::new(from_square, to_square, None, Some(Type::Rook)));
            buffer.push(Move::new(from_square, to_square, None, Some(Type::Bishop)));
        } else {
            buffer.push(Move::new(from_square, to_square, None, None));
        }
    }

    // Generate double pawn moves.
    for to_square in double_moves.iter_bits() {
        let from_square = if turn_idx == BLACK {
            to_square + 16
        } else {
            to_square - 16
        };
        buffer.push(Move::new(from_square, to_square, None, None));
    }

    // Generate en passant captures.
    if state.ep_square.any_set() {
        generate_pseudo_legal_ep(buffer, state, turn_idx, from_mask, to_mask);
    }
}

fn generate_pseudo_legal_ep(
    buffer: &mut MoveBuf,
    state: &Bitboard,
    turn_idx: usize,
    from_mask: u64,
    to_mask: u64,
) {
    if !state.ep_square.any_set() || !(state.ep_square & to_mask).any_set() {
        return;
    }

    if (state.ep_square & state.occupied()).any_set() {
        return;
    }

    let capturers = state.pawns
        & state.occupied_co[turn_idx]
        & from_mask
        & BB_PAWN_ATTACKS[1 ^ turn_idx][state.ep_square.lsb()]
        & BB_RANKS[if turn_idx == WHITE { 4 } else { 3 }];

    for capturer in capturers.iter_bits() {
        buffer.push(Move::new(
            capturer,
            state.ep_square.lsb(),
            Some(Type::Pawn),
            None,
        ));
    }
}

fn attacks_mask(state: &Bitboard, square: Square) -> u64 {
    let bb_square = into_bb(square);

    if (bb_square & state.pawns).any_set() {
        let color = (bb_square & state.occupied_co[WHITE]).any_set() as usize;
        BB_PAWN_ATTACKS[color][square]
    } else if (bb_square & state.knights).any_set() {
        BB_KNIGHT_ATTACKS[square]
    } else if (bb_square & state.kings).any_set() {
        BB_KING_ATTACKS[square]
    } else {
        let mut attacks = BB_EMPTY;
        if (bb_square & state.bishops).any_set() || (bb_square & state.queens).any_set() {
            attacks |= BB_DIAG_ATTACKS[square][&(BB_DIAG_MASKS[square] & state.occupied())];
        }
        if (bb_square & state.rooks).any_set() || (bb_square & state.queens).any_set() {
            attacks |= BB_RANK_ATTACKS[square][&(BB_RANK_MASKS[square] & state.occupied())]
                | BB_FILE_ATTACKS[square][&(BB_FILE_MASKS[square] & state.occupied())];
        }
        attacks
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboards::Bitboard;
    use crate::board::Board;
    use crate::colour::WHITE;
    use crate::magicnumbers::{BB_FILE_A, BB_FILE_B, BB_FILE_C, BB_FILE_D};
    use crate::movebuffer::MoveBuf;
    use crate::movegen::generate_pseudo_legal_moves;

    #[test]
    fn starting_position_count() {
        let board = Board::new();
        let moves = board.legal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn masking_test() {
        let mut buffer = MoveBuf::new();
        let state = Bitboard::new();
        let turn = WHITE;
        let from_mask = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
        let to_mask = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
        generate_pseudo_legal_moves(&mut buffer, &state, turn, from_mask, to_mask);
        assert_eq!(buffer.len(), 10);
    }

    fn perft(board: &mut Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut count = 0;
        for &m in &board.legal_moves() {
            let mut board = board.clone();
            board.make(m);
            count += perft(&mut board, depth - 1);
            board.unmake();
        }
        count
    }

    #[test]
    fn perft_1() {
        let mut board = Board::new();
        let count = perft(&mut board, 1);
        assert_eq!(count, 20);
    }

    #[test]
    fn perft_2() {
        let mut board = Board::new();
        let count = perft(&mut board, 2);
        assert_eq!(count, 400);
    }

    #[test]
    fn perft_4() {
        let mut board = Board::new();
        let before_copy = board.clone();
        let count = perft(&mut board, 4);
        assert_eq!(count, 197_281);
        assert_eq!(board, before_copy);
    }
}
