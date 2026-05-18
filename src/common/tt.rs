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
    pub score: i16,
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
    // 65536 * 32 * 16 bytes = 32 MB
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
        alpha: i16,
        beta: i16,
        depth: u8,
    ) -> (bool, i16, Option<Move>) {
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
        score: i16,
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

    pub fn adjust_score(score: i16, ply: i32) -> i16 {
        if !Self::is_close_to_checkmate(score) {
            return score;
        }
        score + if score > 0 { ply as i16 } else { -ply as i16 }
    }

    pub fn retrieve_score(score: i16, ply: i32) -> i16 {
        if !Self::is_close_to_checkmate(score) {
            return score;
        }
        score + if score > 0 { -ply as i16 } else { ply as i16 }
    }

    fn is_close_to_checkmate(score: i16) -> bool {
        (MAX_CENTIPAWN_EVAL as i32 - (score as i32).abs()) <= MAX_PLY as i32
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
}
