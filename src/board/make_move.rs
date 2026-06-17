use crate::bitboard::attacks::{FILE_A, FILE_H};
use crate::board::state::{BoardState, CASTLING_CONSTANTS};
use crate::common::castle::Castle;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;
use crate::common::zobrist;

impl BoardState {
    pub fn make_move(&mut self, m: Move) {
        let captured_piece = Piece::None;
        let original_board_hash = self.board_hash;
        let original_en_passant_square = self.en_passant_square;
        let original_castling_rights = self.castle;
        let original_half_move_clock = self.half_move_clock;
        let original_accumulator_white = self.accumulator_white;
        let original_accumulator_black = self.accumulator_black;

        self.board_hash ^=
            zobrist::zobrist_table()[self.get_piece_on(m.source) as usize][m.source as usize];
        let moved_piece = self.remove_piece(m.source, true);
        if moved_piece == Piece::Pawn || m.is_capture() {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        let mut final_moved_piece = moved_piece;
        let mut final_captured_piece = captured_piece;

        if m.is_capture() {
            final_captured_piece = self.handle_capture(m);
        }

        if m.is_promotion() {
            final_moved_piece = m.move_type.promotion_piece();
        }

        if m.is_castle() {
            self.handle_castle(m);
        }

        self.add_piece(m.target, self.side_to_move, final_moved_piece, true);
        self.board_hash ^=
            zobrist::zobrist_table()[self.get_piece_on(m.target) as usize][m.target as usize];

        self.update_castling_rights(m);
        self.update_en_passant(m);
        self.flip_side_to_move();

        self.history.save(
            final_captured_piece,
            original_en_passant_square,
            original_castling_rights,
            original_board_hash,
            original_half_move_clock,
            original_accumulator_white,
            original_accumulator_black,
        );
        self.move_count += 1;
    }

    fn handle_castle(&mut self, m: Move) {
        match m.target {
            Square::C1 => self.move_rook_from(Square::A1, Square::D1, self.side_to_move),
            Square::G1 => self.move_rook_from(Square::H1, Square::F1, self.side_to_move),
            Square::C8 => self.move_rook_from(Square::A8, Square::D8, self.side_to_move),
            Square::G8 => self.move_rook_from(Square::H8, Square::F8, self.side_to_move),
            _ => {}
        }
    }

    fn handle_capture(&mut self, m: Move) -> Piece {
        let target_square = if m.move_type.is_en_passant() {
            self.en_passant_square_for(m)
        } else {
            m.target
        };

        self.board_hash ^= zobrist::zobrist_table()[self.get_piece_on(target_square) as usize]
            [target_square as usize];
        self.half_move_clock = 0;

        self.remove_piece(target_square, true)
    }

    fn flip_side_to_move(&mut self) {
        self.board_hash = zobrist::flip_side_to_move_hashes(self, self.board_hash);
        self.side_to_move = self.side_to_move.other();
    }

    fn update_en_passant(&mut self, m: Move) {
        self.board_hash = zobrist::hash_en_passant(self, self.board_hash);

        // TODO: this needs to be rethought for proper impl (FEN, and legal en passsnt represent EP square differently)
        // https://www.talkchess.com/forum/viewtopic.php?t=33397
        self.en_passant_square = if m.move_type.is_double_push() {
            let t = m.target as usize;
            let adjacent = ((1u64 << (t - 1)) & !FILE_H) | ((1u64 << (t + 1)) & !FILE_A);
            if (self.get_pieces(self.side_to_move.other(), Piece::Pawn) & adjacent).is_not_empty() {
                self.en_passant_square_for(m)
            } else {
                Square::NoSquare
            }
        } else {
            Square::NoSquare
        };
        self.board_hash = zobrist::hash_en_passant(self, self.board_hash);
    }

    fn update_castling_rights(&mut self, m: Move) {
        self.board_hash = zobrist::hash_castling_rights(self, self.board_hash);
        self.castle &= Castle::from_bits_retain(CASTLING_CONSTANTS[m.source as usize]);
        self.castle &= Castle::from_bits_retain(CASTLING_CONSTANTS[m.target as usize]);
        self.board_hash = zobrist::hash_castling_rights(self, self.board_hash);
    }

    fn move_rook_from(&mut self, source: Square, target: Square, side: Side) {
        let rook_index = self.get_piece_on(source);
        self.remove_piece(source, true);
        self.add_piece(target, side, Piece::Rook, true);

        self.board_hash ^= zobrist::zobrist_table()[rook_index as usize][source as usize];
        self.board_hash ^= zobrist::zobrist_table()[rook_index as usize][target as usize];
    }

    pub fn unmake_move(&mut self, m: Move) {
        let history = self.history.restore();

        let moved_piece = self.remove_piece(m.target, false);
        self.side_to_move = self.side_to_move.other();

        if history.captured_piece != Piece::None {
            if m.move_type.is_en_passant() {
                self.add_piece(
                    self.en_passant_square_for(m),
                    self.side_to_move.other(),
                    Piece::Pawn,
                    false,
                );
            } else {
                self.add_piece(
                    m.target,
                    self.side_to_move.other(),
                    history.captured_piece,
                    false,
                );
            }
        }

        if m.is_castle() {
            match m.target {
                Square::C1 => {
                    self.remove_piece(Square::D1, false);
                    self.add_piece(Square::A1, self.side_to_move, Piece::Rook, false);
                }
                Square::G1 => {
                    self.remove_piece(Square::F1, false);
                    self.add_piece(Square::H1, self.side_to_move, Piece::Rook, false);
                }
                Square::C8 => {
                    self.remove_piece(Square::D8, false);
                    self.add_piece(Square::A8, self.side_to_move, Piece::Rook, false);
                }
                Square::G8 => {
                    self.remove_piece(Square::F8, false);
                    self.add_piece(Square::H8, self.side_to_move, Piece::Rook, false);
                }
                _ => {}
            }
        }

        self.add_piece(
            m.source,
            self.side_to_move,
            if m.is_promotion() {
                Piece::Pawn
            } else {
                moved_piece
            },
            false,
        );
        self.half_move_clock = history.half_move_clock;
        self.board_hash = history.board_hash;
        self.castle = history.castling_rights;
        self.en_passant_square = history.en_passant_square;
        self.move_count -= 1;
        self.accumulator_black = history.acc_black;
        self.accumulator_white = history.acc_white;
    }

    fn en_passant_square_for(&self, m: Move) -> Square {
        let offset = if self.side_to_move == Side::Black {
            -8
        } else {
            8
        };
        Square::from((m.target as i32 + offset) as usize)
    }

    pub fn is_draw(&self) -> bool {
        let num_pieces = self.occupancy().count_ones();
        if num_pieces == 2 {
            // Assumes a legal board with 2 Kings only
            return true;
        } else if num_pieces == 3 {
            let knights = self.pieces[Piece::Knight];
            let bishops = self.pieces[Piece::Bishop];
            if (knights | bishops).is_not_empty() {
                return true;
            }
        }

        if self.half_move_clock >= 100 {
            return true;
        }
        if self.half_move_clock <= 7 {
            return false;
        }
        self.history.has_hash_appeared_twice(
            self.board_hash,
            self.history
                .index
                .saturating_sub(self.half_move_clock as usize),
        )
    }

    pub fn make_null_move(&mut self) {
        self.history.save(
            Piece::None,
            self.en_passant_square,
            self.castle,
            self.board_hash,
            self.half_move_clock,
            // TODO: value doesn't matter for null move
            self.accumulator_white,
            self.accumulator_black,
        );
        self.update_en_passant(Move::NO_MOVE);
        self.flip_side_to_move();
    }

    pub fn undo_null_move(&mut self) {
        let history = self.history.restore();
        self.flip_side_to_move();
        self.half_move_clock = history.half_move_clock;
        self.board_hash = history.board_hash;
        self.castle = history.castling_rights;
        self.en_passant_square = history.en_passant_square;
        self.accumulator_white = history.acc_white;
        self.accumulator_black = history.acc_black;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;
    use crate::common::helpers::STARTING_FEN;
    use crate::common::move_list::MoveList;
    use crate::common::move_type::MoveType;

    #[test]
    fn test_should_make_and_undo_null_move_correctly() {
        let mut board_state = BoardState::parse_fen(STARTING_FEN);
        let original_state_pieces = board_state.pieces;
        let original_state_side = board_state.side_to_move;
        let original_board_hash = board_state.board_hash;

        board_state.make_null_move();

        assert_eq!(board_state.pieces, original_state_pieces);
        assert_ne!(board_state.side_to_move, original_state_side);
        assert_ne!(board_state.board_hash, original_board_hash);

        board_state.undo_null_move();

        assert_eq!(board_state.pieces, original_state_pieces);
        assert_eq!(board_state.side_to_move, original_state_side);
        assert_eq!(board_state.board_hash, original_board_hash);
    }

    #[test]
    fn test_zobrist_hashing_restore() {
        let cases = vec![
            // Quiet, Captures & Promotions
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "e2e4",
            ),
            (
                "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
                "e4d5",
            ),
            (
                "rnbqkbnr/ppp2ppp/8/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 1",
                "d5e6",
            ),
            (
                "rnbqkbnr/ppppp1P1/8/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 1",
                "g7h8q",
            ),
            // En Passant
            (
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
                "d7d5",
            ),
            (
                "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2",
                "e5d6",
            ),
            (
                "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2",
                "e5e6",
            ),
            // Castling Rights
            ("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1", "e1g1"),
            ("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1", "e1c1"),
            ("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1", "e8g8"),
            ("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1", "e8c8"),
        ];

        for (fen, move_str) in cases {
            let mut board_state = BoardState::parse_fen(fen);
            let mut move_list = MoveList::new();
            board_state.generate_moves(&mut move_list);

            let parsed_move = Move::parse_long_algebraic(move_str).unwrap();
            let mut found_move = Move::NO_MOVE;
            for m in move_list.iter() {
                if m.mv.source == parsed_move.source
                    && m.mv.target == parsed_move.target
                    && (parsed_move.move_type == MoveType::Quiet
                        || ((m.mv.move_type.value() & !8) == parsed_move.move_type.value()))
                {
                    found_move = m.mv;
                    break;
                }
            }
            assert_ne!(
                found_move,
                Move::NO_MOVE,
                "Move {} not found in FEN {}",
                move_str,
                fen
            );

            let original_hash = board_state.board_hash;

            board_state.make_move(found_move);
            assert_eq!(
                zobrist::get_board_hash(&board_state),
                board_state.board_hash,
                "Incremental hash mismatch after make_move for {} in {}",
                move_str,
                fen
            );

            board_state.unmake_move(found_move);
            assert_eq!(
                original_hash, board_state.board_hash,
                "Hash not restored after unmake_move for {} in {}",
                move_str, fen
            );
            assert_eq!(
                zobrist::get_board_hash(&board_state),
                board_state.board_hash,
                "State hash mismatch after unmake_move for {} in {}",
                move_str,
                fen
            );
        }
    }

    #[test]
    fn test_is_draw_fifty_move_rule() {
        let moves_str = "d2d4 g8f6 g1f3 g7g6 c1f4 d7d6 b1d2 f6h5 f4g5 f7f6 g5e3 e7e5 d4d5 f8e7 e3h6 c7c6 e2e4 b8d7 d5c6 b7c6 f1c4 \
                       c8b7 d2b3 a7a5 e1g1 a5a4 b3d2 d6d5 c4d3 d7c5 a1b1 h5f4 h6f4 e5f4 d1e2 d8d7 f1e1 c5d3 e2d3 e8g8 b1d1 a8e8 \
                       d3d4 c6c5 d4d3 d7e6 e4d5 e6d5 d3a3 b7c6 h2h3 g8g7 g1h2 f8f7 a3c3 e8d8 c3c4 d5c4 d2c4 h7h6 d1d8 e7d8 f3d2 f7e7 \
                       e1d1 c6d5 c4d6 d5a2 d2e4 e7e5 e4c3 a2g8 c3a4 d8e7 d6c8 e7f8 d1d7 g8f7 a4c3 g6g5 c3b5 g7g8 d7d2 e5d5 d2d5 f7d5 \
                       c8d6 g8h7 g2g3 h7g6 g3g4 d5c6 h2g1 h6h5 g4h5 g6h5 g1h2 c6d7 h2g2 h5h4 b2b3 f8e7 c2c4 d7h3 g2f3 f6f5 f3e2 h4g4 d6f7 \
                       e7f6 f7h6 g4h5 h6f7 g5g4 f7d6 h5g6 d6b7 f6e7 b5c3 h3g2 c3d5 g2f3 e2d2 f3d5 c4d5 g4g3 f2g3 f4g3 d2e2 e7h4 e2f3 g6f6 b7c5 \
                       f6e5 c5d3 e5d5 d3f4 d5d4 f4e2 d4e5 b3b4 g3g2 b4b5 h4d8 f3g2 e5e4 e2g3 e4f4 g3h5 f4g4 h5g3 f5f4 g3e4 f4f3 g2f1 \
                       g4f5 e4d6 f5f4 d6c4 f4e4 b5b6 e4d5 b6b7 d8c7 c4e3 d5c6 f1f2 c6b7 f2f3 b7c6 f3e4 c6c5 e3d5 c7a5 d5e7 a5b4 \
                       e7g8 b4d2 g8e7 d2b4 e7g8 b4d2 g8f6 d2e1 f6e8 e1c3 e8c7 c3f6 c7e8 f6b2 e8c7 b2f6 c7e6 c5d6 e6d4 d6c5 d4e6 c5c6 \
                       e6d4 c6d6 d4b5 d6c5 b5c7 f6b2 c7a6 c5c4 a6c7 c4c5 c7e6 c5d6 e6g5 b2a1 g5f7 d6c5 f7d8 c5c4 d8f7 c4c5 f7h6 a1c3 \
                       h6f5 c3f6 f5h6 f6c3 h6f5 c3f6 f5e3 f6g7 e3d5 c5d6 d5b4 g7f8 b4d5 f8g7 d5b4 g7f8 b4d5 f8h6 d5b6 h6g7 b6c8 d6c5 c8e7 \
                       g7b2 e7g8 c5d6 g8h6 d6c6 h6g8 c6d7 g8h6 d7e6 h6f5 b2a1 f5h6 e6d6 h6f5 d6c5 f5h4 a1f6 h4f3 f6c3 f3e5 c5d6 e5f3 c3f6 \
                       f3d4 f6e5";
        let moves: Vec<&str> = moves_str.split_whitespace().collect();
        let mut board_state = BoardState::default();
        for move_str in moves {
            let mut move_list = MoveList::new();
            board_state.generate_moves(&mut move_list);
            let parsed_move = Move::parse_long_algebraic(move_str)
                .unwrap_or_else(|| panic!("Failed to parse move: '{}'", move_str));
            let mut found_move = Move::NO_MOVE;
            for m in move_list.iter() {
                if m.mv.source == parsed_move.source && m.mv.target == parsed_move.target {
                    found_move = m.mv;
                    break;
                }
            }
            board_state.make_move(found_move);
        }
        assert!(!board_state.is_draw());
        let fifty_move = Move::new(Square::D4, Square::C2, MoveType::Quiet);
        board_state.make_move(fifty_move);
        assert!(board_state.is_draw());
    }

    #[test]
    fn test_threefold_with_intervening_pawn_moves() {
        let moves_str = "d2d4 e7e6 g1f3 g8f6 c1f4 c7c5 c2c3 c5d4 c3d4 d8b6 d1c2 b8c6 e2e3 c6b4 c2b3 b4d5 b3b6 d5b6 b1c3 f6d5 f4e5 d5c3 b2c3 f7f6 e5g3 b6d5 a1c1 f8a3 c1c2 e8g8 e3e4 d5e7 f1d3 d7d5 e1g1 d5e4 d3e4 f6f5 e4d3 f5f4 g3h4 a3d6 f3e5 e7f5 h4g5 h7h6 g5f4 f5d4 c3d4 f8f4 f1c1 f4f8 d3e4 f8d8 e4g6 d8f8 g6f7 g8h7 f7g6 h7g8 a2a4 a7a5 g6f7 g8h7 f7g6 h7g8 g6f7 g8h7 f7g6 h7g8";
        let moves: Vec<&str> = moves_str.split_whitespace().collect();
        let mut board = BoardState::default();
        for (i, move_str) in moves.iter().enumerate() {
            let mut move_list = MoveList::new();
            board.generate_moves(&mut move_list);
            let parsed = Move::parse_long_algebraic(move_str).unwrap();
            let mut found = Move::NO_MOVE;
            for m in move_list.iter() {
                if m.mv.source == parsed.source && m.mv.target == parsed.target {
                    found = m.mv;
                    break;
                }
            }
            assert_ne!(found, Move::NO_MOVE, "Move {} ({}) not found", i, move_str);
            board.make_move(found);
        }
        assert!(
            board.is_draw(),
            "Should detect threefold repetition after the full sequence"
        );
    }

    #[test]
    fn test_is_draw_threefold_repetition() {
        let mut board = BoardState::default();

        let nf3 = Move::new(Square::G1, Square::F3, MoveType::Quiet);
        let nf6 = Move::new(Square::G8, Square::F6, MoveType::Quiet);
        let ng1 = Move::new(Square::F3, Square::G1, MoveType::Quiet);
        let ng8 = Move::new(Square::F6, Square::G8, MoveType::Quiet);

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(ng1);
        board.make_move(ng8);
        assert!(!board.is_draw());

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(ng1);
        board.make_move(ng8);
        assert!(board.is_draw());
    }

    #[test]
    fn test_should_not_detect_threefold_repetition_when_moves_are_different() {
        let mut board = BoardState::default();

        let nf3 = Move::new(Square::G1, Square::F3, MoveType::Quiet);
        let nf6 = Move::new(Square::G8, Square::F6, MoveType::Quiet);
        let ne5 = Move::new(Square::F3, Square::E5, MoveType::Quiet);
        let ne4 = Move::new(Square::F6, Square::E4, MoveType::Quiet);
        let back_nf3 = Move::new(Square::E5, Square::F3, MoveType::Quiet);
        let back_nf6 = Move::new(Square::E4, Square::F6, MoveType::Quiet);

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(ne5);
        board.make_move(ne4);
        board.make_move(back_nf3);
        board.make_move(back_nf6);

        assert!(!board.is_draw());
    }

    #[test]
    fn test_reset_repetition_count_after_pawn_move() {
        let mut board = BoardState::default();
        let nf3 = Move::new(Square::G1, Square::F3, MoveType::Quiet);
        let nf6 = Move::new(Square::G8, Square::F6, MoveType::Quiet);
        let ng1 = Move::new(Square::F3, Square::G1, MoveType::Quiet);
        let ng8 = Move::new(Square::F6, Square::G8, MoveType::Quiet);
        let e4 = Move::new(Square::E2, Square::E4, MoveType::DoublePush);

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(ng1);
        board.make_move(ng8);
        assert!(!board.is_draw());

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(e4);
        board.make_move(ng8);
        assert!(!board.is_draw());

        board.make_move(ng1);
        board.make_move(nf6);
        board.make_move(nf3);
        board.make_move(ng8);
        assert!(!board.is_draw());
    }

    #[test]
    fn test_reset_repetition_count_after_capture() {
        let mut board = BoardState::default();
        let e4 = Move::new(Square::E2, Square::E4, MoveType::DoublePush);
        let d5 = Move::new(Square::D7, Square::D5, MoveType::DoublePush);
        let dxe4 = Move::new(Square::D5, Square::E4, MoveType::Capture);

        let nf3 = Move::new(Square::G1, Square::F3, MoveType::Quiet);
        let nf6 = Move::new(Square::G8, Square::F6, MoveType::Quiet);
        let ng1 = Move::new(Square::F3, Square::G1, MoveType::Quiet);
        let ng8 = Move::new(Square::F6, Square::G8, MoveType::Quiet);

        board.make_move(e4);
        board.make_move(d5);

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(ng1);
        board.make_move(ng8);
        assert!(!board.is_draw());

        board.make_move(nf3);
        board.make_move(nf6);
        board.make_move(ng1);
        assert!(!board.is_draw());

        board.make_move(dxe4);

        board.make_move(nf3);
        board.make_move(ng8);
        board.make_move(ng1);
        board.make_move(nf6);
        assert!(!board.is_draw());

        board.make_move(nf3);
        board.make_move(ng8);
        board.make_move(ng1);
        board.make_move(nf6);
        assert!(board.is_draw());
    }

    #[test]
    fn test_is_draw_insufficient_material() {
        // KvK
        let board = BoardState::parse_fen("8/8/8/8/8/8/8/4K2k w - - 0 1");
        assert!(board.is_draw());

        // KvKB
        let board = BoardState::parse_fen("8/8/8/8/8/8/8/4K1Bk w - - 0 1");
        assert!(board.is_draw());

        // KvKN
        let board = BoardState::parse_fen("8/8/8/8/8/8/8/4K1Nk w - - 0 1");
        assert!(board.is_draw());

        // KBvK
        let board = BoardState::parse_fen("8/8/8/8/8/8/8/4K1bk w - - 0 1");
        assert!(board.is_draw());

        // KNvK
        let board = BoardState::parse_fen("8/8/8/8/8/8/8/4K1nk w - - 0 1");
        assert!(board.is_draw());

        // KvKBB (not a forced draw)
        let board = BoardState::parse_fen("8/8/8/8/8/8/8/4KBBk w - - 0 1");
        assert!(!board.is_draw());

        // KvP
        let board = BoardState::parse_fen("8/8/8/8/8/8/4P3/4K2k w - - 0 1");
        assert!(!board.is_draw());

        // KvR
        let board = BoardState::parse_fen("8/8/8/8/8/8/4R3/4K2k w - - 0 1");
        assert!(!board.is_draw());

        // KvQ
        let board = BoardState::parse_fen("8/8/8/8/8/8/4Q3/4K2k w - - 0 1");
        assert!(!board.is_draw());
    }

    #[test]
    fn should_not_reset_half_move_clock_after_castle() {
        let mut board = BoardState::parse_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let original_half_move_clock = board.half_move_clock;
        board.make_move(Move::new(Square::E1, Square::G1, MoveType::Castle));
        assert_eq!(board.half_move_clock, original_half_move_clock + 1);
    }

    #[test]
    fn should_reset_half_move_clock_after_en_passant() {
        let mut board =
            BoardState::parse_fen("rnbqkbnr/pppp1ppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2");

        // Quiet Moves which won't update draw killer
        board.make_move(Move {
            source: Square::B7,
            target: Square::C6,
            move_type: MoveType::Quiet,
        });
        board.make_move(Move {
            source: Square::B1,
            target: Square::C3,
            move_type: MoveType::Quiet,
        });
        let mut move_list = MoveList::new();
        board.generate_moves(&mut move_list);
        let double_push = move_list
            .iter()
            .copied()
            .find(|m| m.mv.source == Square::F7 && m.mv.target == Square::F5)
            .expect("f7f5 double push must exist");
        board.make_move(double_push.mv);
        assert_eq!(board.en_passant_square, Square::F6);
        assert_eq!(board.half_move_clock, 0);
    }
}
