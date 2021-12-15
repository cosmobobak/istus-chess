#[cfg(test)]
mod bitmethods_tests {
    use crate::bitmethods::Bithackable;

    #[test]
    fn to_vec() {
        let bb = 0b0110_0110_0110_0110_u64;
        let vec = bb.to_vec();
        assert_eq!(vec, vec![1, 2, 5, 6, 9, 10, 13, 14]);
    }
}

#[cfg(test)]
mod colour_tests {
    use crate::colour::Colour;
    use Colour::{Black, White};

    #[test]
    fn colour_creation() {
        let byte = 0_u8;
        assert_eq!(White, byte.into());
        let short = 0_u16;
        assert_eq!(White, short.into());
        let int = 0_u32;
        assert_eq!(White, int.into());

        let byte = 1_u8;
        assert_eq!(Black, byte.into());
        let short = 1_u16;
        assert_eq!(Black, short.into());
        let int = 1_u32;
        assert_eq!(Black, int.into());
    }

    #[test]
    fn colour_modulus() {
        let byte = 160_u8;
        assert_eq!(White, byte.into());
        let short = 160_u16;
        assert_eq!(White, short.into());
        let int = 160_u32;
        assert_eq!(White, int.into());

        let byte = 161_u8;
        assert_eq!(Black, byte.into());
        let short = 161_u16;
        assert_eq!(Black, short.into());
        let int = 161_u32;
        assert_eq!(Black, int.into());
    }

    #[test]
    fn colour_to_int() {
        let white = White;
        assert_eq!(0, white as u32);

        let black = Black;
        assert_eq!(1, black as u32);
    }
}

#[cfg(test)]
mod move_tests {
    use crate::cmove::Move;
    use crate::piece::Type;
    use crate::squares::Square;
    use Square::{A7, A8, E2, E4};

    #[test]
    fn uci() {
        let m = Move::from_uci("e2e4").unwrap();
        let from = m.from_sq();
        let to = m.to_sq();
        assert_eq!(from, E2);
        assert_eq!(to, E4);
    }

    #[test]
    fn uci_promo() {
        let m = Move::from_uci("a7a8q").unwrap();
        let from = m.from_sq();
        let to = m.to_sq();
        assert_eq!(from, A7);
        assert_eq!(to, A8);
        assert_eq!(m.promotion(), Some(Type::Queen));
    }

    #[test]
    fn uci_invalid() {
        let m = Move::from_uci("e2e4e5");
        assert!(m.is_err());
        let m = Move::from_uci("e2e9");
        assert!(m.is_err());
        let m = Move::from_uci("2e4e");
        assert!(m.is_err());
        let m = Move::from_uci("e2e4q");
        assert!(m.is_err());
        let m = Move::from_uci("j2e4");
        assert!(m.is_err());
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
    use crate::squares::Square;
    use Square::{E2, E4, F5, G7, G8};
    use crate::board::Board;

    #[test]
    fn make_move() {
        let mut board = Board::new();
        board.make(Move::from_uci("e2e4").unwrap());
        assert_eq!(board.get_piece_at(E4).unwrap().piece_type, Type::Pawn);
        assert_eq!(board.get_piece_at(E2), None);
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
        assert_eq!(board.get_piece_at(G8), Some(Piece::new(Type::Queen, Colour::White)));
        assert_eq!(board.get_piece_at(G7), None);
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8n").unwrap());
        assert_eq!(board.get_piece_at(G8), Some(Piece::new(Type::Knight, Colour::White)));
        assert_eq!(board.get_piece_at(G7), None);
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8r").unwrap());
        assert_eq!(board.get_piece_at(G8), Some(Piece::new(Type::Rook, Colour::White)));
        assert_eq!(board.get_piece_at(G7), None);
        let mut board = refboard.clone();
        board.make(Move::from_uci("g7g8b").unwrap());
        assert_eq!(board.get_piece_at(G8), Some(Piece::new(Type::Bishop, Colour::White)));
        assert_eq!(board.get_piece_at(G7), None);
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
        assert_eq!(board.get_piece_at(F5), None);
        board.unmake();
        println!("{}", board);
        assert_eq!(board.get_piece_at(F5), Some(Piece::new(Type::Pawn, Colour::Black)));
    }
}

#[cfg(test)]
mod move_generation {
    use crate::bitboards::{Bitboard, BB_FILE_A, BB_FILE_B, BB_FILE_C, BB_FILE_D};
    use crate::colour::Colour;
    use crate::movegen::generate_pseudo_legal_moves;
    use crate::board::Board;

    #[test]
    fn starting_position_count() {
        let board = Board::new();
        let moves = board.legal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn masking_test() {
        let mut buffer = Vec::with_capacity(256);
        let state = Bitboard::new();
        let turn = Colour::White;
        let from_mask = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
        let to_mask = BB_FILE_A | BB_FILE_B | BB_FILE_C | BB_FILE_D;
        generate_pseudo_legal_moves(&mut buffer, &state, turn, from_mask, to_mask);
        assert_eq!(buffer.len(), 10);
    }
    
    fn perft(board: &mut Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = board.legal_moves();
        let mut count = 0;
        for m in moves {
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
}