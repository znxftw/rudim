use crate::board::state::BoardState;
use crate::common::constants::{MAX_CENTIPAWN_EVAL, MAX_PLY};
use crate::common::moves::Move;
use std::sync::LazyLock;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranspositionEntryType {
    Exact,
    Alpha,
    Beta,
}

#[derive(Debug, Clone, Copy)]
pub struct TranspositionTableEntry {
    pub score: i32,
    pub hash: u64,
    pub depth: u8,
    pub best_move: Move,
    pub entry_type: TranspositionEntryType,
}

pub struct TranspositionTable {
    entries: Vec<Option<TranspositionTableEntry>>,
    capacity: usize,
}

impl TranspositionTable {
    // 65536 * 32 * 24 bytes = 48 MB
    pub const DEFAULT_CAPACITY: usize = 65536 * 32;

    pub fn new(capacity: usize) -> Self {
        assert!(
            capacity.is_power_of_two(),
            "Capacity must be a power of two"
        );
        Self {
            entries: vec![None; capacity],
            capacity,
        }
    }

    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = None;
        }
    }

    pub fn get_hash_move(&self, hash: u64) -> Option<Move> {
        let entry = self.entries[(hash as usize) & (self.capacity - 1)]?;
        if entry.hash == hash && entry.entry_type == TranspositionEntryType::Exact {
            Some(entry.best_move)
        } else {
            None
        }
    }

    pub fn get_entry(
        &self,
        hash: u64,
        alpha: i32,
        beta: i32,
        depth: u8,
    ) -> (bool, i32, Option<Move>) {
        let entry = match self.entries[(hash as usize) & (self.capacity - 1)] {
            Some(e) => e,
            None => return (false, 0, None),
        };

        if entry.hash != hash {
            return (false, 0, None);
        }

        if entry.depth < depth {
            return (false, 0, Some(entry.best_move));
        }

        match entry.entry_type {
            TranspositionEntryType::Exact => (true, entry.score, Some(entry.best_move)),
            TranspositionEntryType::Alpha => {
                if entry.score <= alpha {
                    (true, alpha, Some(entry.best_move))
                } else {
                    (false, 0, Some(entry.best_move))
                }
            }
            TranspositionEntryType::Beta => {
                if entry.score >= beta {
                    (true, beta, Some(entry.best_move))
                } else {
                    (false, 0, Some(entry.best_move))
                }
            }
        }
    }

    pub fn submit_entry(
        &mut self,
        hash: u64,
        score: i32,
        depth: u8,
        best_move: Move,
        entry_type: TranspositionEntryType,
    ) {
        let index = (hash as usize) & (self.capacity - 1);
        if let Some(existing) = self.entries[index]
            && existing.depth >= depth
        {
            return;
        }

        self.entries[index] = Some(TranspositionTableEntry {
            hash,
            score,
            depth,
            best_move,
            entry_type,
        });
    }

    pub fn collect_principal_variation(&self, board_state: &mut BoardState) -> Vec<Move> {
        let mut pv = Vec::new();

        loop {
            let hash = board_state.board_hash;
            let entry = match self.entries[(hash as usize) & (self.capacity - 1)] {
                Some(e) => e,
                None => break,
            };

            if entry.hash != hash || entry.entry_type != TranspositionEntryType::Exact {
                break;
            }

            if pv.contains(&entry.best_move) {
                break;
            }

            // TODO: revisit, is this needed?
            if entry.best_move == Move::NO_MOVE {
                break;
            }

            board_state.make_move(entry.best_move);
            if board_state.is_in_check(board_state.side_to_move.other()) {
                board_state.unmake_move(entry.best_move);
                break;
            }

            pv.push(entry.best_move);

            if board_state.is_draw() {
                break;
            }
        }

        // Unmake moves in reverse order
        for m in pv.iter().rev() {
            board_state.unmake_move(*m);
        }

        pv
    }

    pub fn adjust_score(score: i32, ply: i32) -> i32 {
        if !Self::is_close_to_checkmate(score) {
            return score;
        }
        score + if score > 0 { ply } else { -ply }
    }

    pub fn retrieve_score(score: i32, ply: i32) -> i32 {
        if !Self::is_close_to_checkmate(score) {
            return score;
        }
        score + if score > 0 { -ply } else { ply }
    }

    fn is_close_to_checkmate(score: i32) -> bool {
        MAX_CENTIPAWN_EVAL - score.abs() <= MAX_PLY as i32
    }
}

pub static TT: LazyLock<Mutex<TranspositionTable>> = LazyLock::new(|| {
    Mutex::new(TranspositionTable::new(
        TranspositionTable::DEFAULT_CAPACITY,
    ))
});

#[cfg(test)]
mod tests {
    // TODO: revisit, improve tests for more accurate scenarios
    use super::*;
    use crate::board::state::BoardState;
    use crate::common::move_type::MoveType;
    use crate::common::square::Square;

    #[test]
    fn test_tt_store_retrieve() {
        let mut tt = TranspositionTable::new(1024);
        let hash = 123456789;
        let best_move = Move::new(Square::E2, Square::E4, MoveType::Quiet);

        tt.submit_entry(hash, 100, 5, best_move, TranspositionEntryType::Exact);

        let (found, score, m) = tt.get_entry(hash, -1000, 1000, 5);
        assert!(found);
        assert_eq!(score, 100);
        assert_eq!(m, Some(best_move));
    }

    #[test]
    fn test_tt_depth_priority() {
        let mut tt = TranspositionTable::new(1024);
        let hash = 123456789;
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let m2 = Move::new(Square::D2, Square::D4, MoveType::Quiet);

        tt.submit_entry(hash, 100, 5, m1, TranspositionEntryType::Exact);
        tt.submit_entry(hash, 200, 3, m2, TranspositionEntryType::Exact); // Should NOT overwrite

        let (_, score, m) = tt.get_entry(hash, -1000, 1000, 1);
        assert_eq!(score, 100);
        assert_eq!(m, Some(m1));

        tt.submit_entry(hash, 300, 10, m2, TranspositionEntryType::Exact); // Should overwrite
        let (_, score, m) = tt.get_entry(hash, -1000, 1000, 1);
        assert_eq!(score, 300);
        assert_eq!(m, Some(m2));
    }

    #[test]
    fn test_tt_score_adjustment() {
        let mate_score = MAX_CENTIPAWN_EVAL - 5;
        let adjusted = TranspositionTable::adjust_score(mate_score, 10);
        assert_eq!(adjusted, mate_score + 10);

        let retrieved = TranspositionTable::retrieve_score(adjusted, 10);
        assert_eq!(retrieved, mate_score);
    }

    #[test]
    fn test_collect_principal_variation_stops_at_draw() {
        let mut tt = TranspositionTable::new(1024);
        let mut board = BoardState::parse_fen("7k/8/8/8/8/4n3/3B4/4K3 w - - 0 1");

        let first_move = Move::new(Square::D2, Square::E3, MoveType::Capture);
        let root_hash = board.board_hash;
        tt.submit_entry(root_hash, 0, 4, first_move, TranspositionEntryType::Exact);

        board.make_move(first_move);
        assert!(board.is_draw(), "K+B vs K should be recognized as draw");
        let draw_hash = board.board_hash;

        let second_move = Move::new(Square::H8, Square::G8, MoveType::Quiet);
        tt.submit_entry(draw_hash, 0, 3, second_move, TranspositionEntryType::Exact);

        board.unmake_move(first_move);

        let pv = tt.collect_principal_variation(&mut board);
        assert_eq!(pv, vec![first_move]);
    }
}
