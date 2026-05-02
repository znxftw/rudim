use crate::common::castle::Castle;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::square::Square;

pub const HISTORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardHistory {
    pub captured_piece: Piece,
    pub en_passant_square: Square,
    pub castling_rights: Castle,
    pub board_hash: u64,
    pub last_draw_killer: i32,
    pub best_move: Move,
}

impl Default for BoardHistory {
    fn default() -> Self {
        Self {
            captured_piece: Piece::None,
            en_passant_square: Square::NoSquare,
            castling_rights: Castle::NONE,
            board_hash: 0,
            last_draw_killer: 0,
            best_move: Move::NO_MOVE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct History {
    pub entries: [BoardHistory; HISTORY_SIZE],
    pub index: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: [BoardHistory::default(); HISTORY_SIZE],
            index: 0,
        }
    }

    pub fn save(
        &mut self,
        captured_piece: Piece,
        en_passant: Square,
        original_castling_rights: Castle,
        board_hash: u64,
        last_draw_killer: i32,
        best_move: Move,
    ) {
        if self.index < HISTORY_SIZE {
            self.entries[self.index] = BoardHistory {
                captured_piece,
                en_passant_square: en_passant,
                castling_rights: original_castling_rights,
                board_hash,
                last_draw_killer,
                best_move,
            };
            self.index += 1;
        } else {
            panic!("History stack overflow");
        }
    }

    pub fn restore(&mut self) -> BoardHistory {
        if self.index > 0 {
            self.index -= 1;
            self.entries[self.index]
        } else {
            panic!("History stack underflow");
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.index {
            self.entries[i] = BoardHistory::default();
        }
        self.index = 0;
    }

    pub fn has_hash_appeared_twice(&self, board_hash: u64, starting_index: usize) -> bool {
        let mut count = 0;

        for i in (starting_index..self.index).rev() {
            if self.entries[i].board_hash == board_hash {
                count += 1;
            }

            if count == 2 {
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::move_type::MoveType;

    #[test]
    fn test_history_save_restore() {
        let mut history = History::new();
        let best_move = Move::new(Square::E2, Square::E4, MoveType::Quiet);

        history.save(
            Piece::Pawn,
            Square::E3,
            Castle::WHITE_SHORT,
            123456789,
            42,
            best_move,
        );

        assert!(!history.is_empty());
        assert_eq!(history.index, 1);

        let restored = history.restore();
        assert_eq!(restored.captured_piece, Piece::Pawn);
        assert_eq!(restored.en_passant_square, Square::E3);
        assert_eq!(restored.castling_rights, Castle::WHITE_SHORT);
        assert_eq!(restored.board_hash, 123456789);
        assert_eq!(restored.last_draw_killer, 42);
        assert_eq!(restored.best_move, best_move);
        assert!(history.is_empty());
    }

    #[test]
    fn test_history_clear() {
        let mut history = History::new();
        history.save(
            Piece::None,
            Square::NoSquare,
            Castle::NONE,
            0,
            0,
            Move::NO_MOVE,
        );
        assert!(!history.is_empty());
        history.clear();
        assert!(history.is_empty());
        assert_eq!(history.index, 0);
    }

    #[test]
    fn test_has_hash_appeared_twice() {
        let mut history = History::new();
        let hash = 0xDEADBEEF;

        history.save(
            Piece::None,
            Square::NoSquare,
            Castle::NONE,
            hash,
            0,
            Move::NO_MOVE,
        );
        history.save(
            Piece::None,
            Square::NoSquare,
            Castle::NONE,
            0x123,
            0,
            Move::NO_MOVE,
        );
        history.save(
            Piece::None,
            Square::NoSquare,
            Castle::NONE,
            hash,
            0,
            Move::NO_MOVE,
        );

        assert!(history.has_hash_appeared_twice(hash, 0));
        assert!(!history.has_hash_appeared_twice(0x123, 0));
        assert!(!history.has_hash_appeared_twice(hash, 1));
    }

    #[test]
    fn test_should_save_and_restore_board_history() {
        use crate::board::state::BoardState;
        use crate::common::helpers::STARTING_FEN;
        use crate::common::move_type::MoveType;

        let mut board_state = BoardState::parse_fen(STARTING_FEN);
        let original_state_pieces = board_state.pieces.clone();
        let original_state_side = board_state.side_to_move;
        let original_board_hash = board_state.board_hash;
        
        let move_e2e4 = Move::new(Square::E2, Square::E4, MoveType::Quiet);

        board_state.make_move(move_e2e4);

        assert_ne!(board_state.pieces, original_state_pieces);
        assert_ne!(board_state.side_to_move, original_state_side);
        // assert_ne!(board_state.board_hash, original_board_hash); // TODO: Phase 6 Zobrist

        board_state.unmake_move(move_e2e4);

        assert_eq!(board_state.pieces, original_state_pieces);
        assert_eq!(board_state.side_to_move, original_state_side);
        assert_eq!(board_state.board_hash, original_board_hash);
    }
}
