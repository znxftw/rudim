use crate::board::state::{BoardState, CASTLING_CONSTANTS};
use crate::common::castle::Castle;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::side::Side;
use crate::common::square::Square;

// NOTE: Zobrist hashing is currently implemented with placeholders or direct field updates
// because Phase 6 (Zobrist) has not been reached yet. However, for MakeMove/UnmakeMove to work
// as a 1:1 port, we need to handle hash updates if they were part of the C# code.
// Looking at the migration plan, Zobrist is Phase 6. I'll add TODOs or minimal support.

impl BoardState {
    pub fn make_move(&mut self, m: Move) {
        let captured_piece = Piece::None;
        let original_board_hash = self.board_hash;
        let original_en_passant_square = self.en_passant_square;
        let original_castling_rights = self.castle;
        let original_last_draw_killer = self.last_draw_killer;

        // self.board_hash ^= Zobrist::ZobristTable[self.get_piece_on(m.source), m.source as usize]; // Phase 6
        let moved_piece = self.remove_piece(m.source);
        if moved_piece == Piece::Pawn {
            self.last_draw_killer = self.move_count;
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

        self.add_piece(m.target, self.side_to_move, final_moved_piece);
        // self.board_hash ^= Zobrist::ZobristTable[self.get_piece_on(m.target), m.target as usize]; // Phase 6

        self.update_castling_rights(m);
        self.update_en_passant(m);
        self.flip_side_to_move();

        self.history.save(
            final_captured_piece,
            original_en_passant_square,
            original_castling_rights,
            original_board_hash,
            original_last_draw_killer,
            self.best_move,
        );
        self.best_move = Move::NO_MOVE;
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

        // self.board_hash ^= Zobrist::ZobristTable[self.get_piece_on(target_square), target_square as usize]; // Phase 6
        self.last_draw_killer = self.move_count;

        self.remove_piece(target_square)
    }

    fn flip_side_to_move(&mut self) {
        // self.board_hash = Zobrist::flip_side_to_move_hashes(self, self.board_hash); // Phase 6
        self.side_to_move = self.side_to_move.other();
    }

    fn update_en_passant(&mut self, m: Move) {
        let original_en_passant_square = self.en_passant_square;
        // self.board_hash = Zobrist::hash_en_passant(self, self.board_hash); // Phase 6
        self.en_passant_square = if m.move_type.is_double_push() {
            self.en_passant_square_for(m)
        } else {
            Square::NoSquare
        };
        // self.board_hash = Zobrist::hash_en_passant(self, self.board_hash); // Phase 6
        if original_en_passant_square != self.en_passant_square {
            self.last_draw_killer = self.move_count;
        }
    }

    fn update_castling_rights(&mut self, m: Move) {
        let original_castling_rights = self.castle;
        // self.board_hash = Zobrist::hash_castling_rights(self, self.board_hash); // Phase 6
        self.castle &= Castle::from_bits_retain(CASTLING_CONSTANTS[m.source as usize]);
        self.castle &= Castle::from_bits_retain(CASTLING_CONSTANTS[m.target as usize]);
        // self.board_hash = Zobrist::hash_castling_rights(self, self.board_hash); // Phase 6
        if self.castle != original_castling_rights {
            self.last_draw_killer = self.move_count;
        }
    }

    fn move_rook_from(&mut self, source: Square, target: Square, side: Side) {
        self.remove_piece(source);
        self.add_piece(target, side, Piece::Rook);

        // Phase 6: Board hash updates for rook
        // let rook_index = self.get_piece_on(target);
        // self.board_hash ^= Zobrist::ZobristTable[rook_index, source as usize];
        // self.board_hash ^= Zobrist::ZobristTable[rook_index, target as usize];
    }

    pub fn unmake_move(&mut self, m: Move) {
        let history = self.history.restore();

        let moved_piece = self.remove_piece(m.target);
        self.side_to_move = self.side_to_move.other();

        if history.captured_piece != Piece::None {
            if m.move_type.is_en_passant() {
                self.add_piece(
                    self.en_passant_square_for(m),
                    self.side_to_move.other(),
                    Piece::Pawn,
                );
            } else {
                self.add_piece(m.target, self.side_to_move.other(), history.captured_piece);
            }
        }

        if m.is_castle() {
            match m.target {
                Square::C1 => {
                    self.remove_piece(Square::D1);
                    self.add_piece(Square::A1, self.side_to_move, Piece::Rook);
                }
                Square::G1 => {
                    self.remove_piece(Square::F1);
                    self.add_piece(Square::H1, self.side_to_move, Piece::Rook);
                }
                Square::C8 => {
                    self.remove_piece(Square::D8);
                    self.add_piece(Square::A8, self.side_to_move, Piece::Rook);
                }
                Square::G8 => {
                    self.remove_piece(Square::F8);
                    self.add_piece(Square::H8, self.side_to_move, Piece::Rook);
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
        );
        self.last_draw_killer = history.last_draw_killer;
        self.board_hash = history.board_hash;
        self.castle = history.castling_rights;
        self.en_passant_square = history.en_passant_square;
        self.best_move = history.best_move;
        self.move_count -= 1;
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
        if self.move_count - self.last_draw_killer > 100 {
            return true;
        }
        if self.move_count - self.last_draw_killer <= 7 {
            return false;
        }
        self.history
            .has_hash_appeared_twice(self.board_hash, self.last_draw_killer as usize)
    }

    pub fn make_null_move(&mut self) {
        self.history.save(
            Piece::None,
            self.en_passant_square,
            self.castle,
            self.board_hash,
            self.last_draw_killer,
            self.best_move,
        );
        self.update_en_passant(Move::NO_MOVE);
        self.flip_side_to_move();
    }

    pub fn undo_null_move(&mut self) {
        let history = self.history.restore();
        self.flip_side_to_move();
        self.last_draw_killer = history.last_draw_killer;
        self.board_hash = history.board_hash;
        self.castle = history.castling_rights;
        self.en_passant_square = history.en_passant_square;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::BoardState;
    use crate::common::helpers::STARTING_FEN;
    use crate::common::move_type::MoveType;

    #[test]
    fn test_should_make_and_undo_null_move_correctly() {
        let mut board_state = BoardState::parse_fen(STARTING_FEN);
        let original_state_pieces = board_state.pieces.clone();
        let original_state_side = board_state.side_to_move;
        let original_board_hash = board_state.board_hash;

        board_state.make_null_move();

        assert_eq!(board_state.pieces, original_state_pieces);
        assert_ne!(board_state.side_to_move, original_state_side);
        // assert_ne!(board_state.board_hash, original_board_hash); // TODO: Phase 6 Zobrist

        board_state.undo_null_move();

        assert_eq!(board_state.pieces, original_state_pieces);
        assert_eq!(board_state.side_to_move, original_state_side);
        assert_eq!(board_state.board_hash, original_board_hash);
    }

    #[test]
    fn test_is_draw_fifty_move_rule() {
        let mut board_state = BoardState::parse_fen(STARTING_FEN);
        board_state.last_draw_killer = 0;
        board_state.move_count = 101;
        assert!(board_state.is_draw());

        board_state.move_count = 100;
        assert!(!board_state.is_draw());
    }

    #[test]
    fn test_is_draw_threefold_repetition() {
        let mut board = BoardState::parse_fen(STARTING_FEN);
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
    fn test_reset_repetition_count_after_pawn_move() {
        let mut board = BoardState::parse_fen(STARTING_FEN);
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
        let mut board = BoardState::parse_fen(STARTING_FEN);
        let e4 = Move::new(Square::E2, Square::E4, MoveType::DoublePush);
        let d5 = Move::new(Square::D7, Square::D5, MoveType::DoublePush);
        let exd5 = Move::new(Square::E4, Square::D5, MoveType::Capture);

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

        board.make_move(exd5);
        board.make_move(nf6);
        board.make_move(nf3);
        board.make_move(ng8);
        board.make_move(ng1);
        assert!(!board.is_draw());
    }
}
