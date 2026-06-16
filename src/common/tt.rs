use crate::common::constants::{MAX_CENTIPAWN_EVAL, MAX_PLY};
use crate::common::moves::Move;

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

// TODO: profile, tune
// TODO: bucketed instead of 2-tier
pub struct TranspositionTable {
    depth_replaced_entries: Vec<Option<TranspositionTableEntry>>,
    always_replaced_entries: Vec<Option<TranspositionTableEntry>>,
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
            depth_replaced_entries: vec![None; capacity],
            always_replaced_entries: vec![None; capacity],
            capacity,
        }
    }

    pub fn resize(&mut self, mb_size: usize) {
        let max_bytes = mb_size * 1024 * 1024;
        let entry_size = size_of::<Option<TranspositionTableEntry>>();
        let max_capacity = max_bytes / (2 * entry_size);

        let capacity = if max_capacity < 1024 {
            1024
        } else {
            1 << max_capacity.ilog2()
        };

        self.capacity = capacity;
        self.depth_replaced_entries = vec![None; capacity];
        self.always_replaced_entries = vec![None; capacity];
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        for entry in self.depth_replaced_entries.iter_mut() {
            *entry = None;
        }
        for entry in self.always_replaced_entries.iter_mut() {
            *entry = None;
        }
    }

    pub fn get_entry(
        &self,
        hash: u64,
        alpha: i16,
        beta: i16,
        depth: u8,
        ply: u8,
    ) -> (bool, i16, Option<Move>) {
        let index = (hash as usize) & (self.capacity - 1);

        let mut found_entry = None;
        if let Some(e) = self.depth_replaced_entries[index]
            && e.hash == hash
        {
            found_entry = Some(e);
        }
        if found_entry.is_none()
            && let Some(e) = self.always_replaced_entries[index]
            && e.hash == hash
        {
            found_entry = Some(e);
        }

        let entry = match found_entry {
            Some(e) => e,
            None => return (false, 0, None),
        };

        if entry.depth < depth {
            return (false, 0, Some(entry.best_move));
        }

        let tt_score = Self::retrieve_score(entry.score, ply as i32);

        match entry.entry_type {
            TranspositionEntryType::Exact => (true, tt_score, Some(entry.best_move)),
            TranspositionEntryType::Alpha => {
                if tt_score <= alpha {
                    (true, tt_score, Some(entry.best_move))
                } else {
                    (false, 0, Some(entry.best_move))
                }
            }
            TranspositionEntryType::Beta => {
                if tt_score >= beta {
                    (true, tt_score, Some(entry.best_move))
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
        let new_entry = Some(TranspositionTableEntry {
            hash,
            score,
            depth,
            best_move,
            entry_type,
        });

        if let Some(existing) = self.depth_replaced_entries[index] {
            if existing.depth >= depth {
                self.always_replaced_entries[index] = new_entry;
            } else {
                self.always_replaced_entries[index] = self.depth_replaced_entries[index];
                self.depth_replaced_entries[index] = new_entry;
            }
        } else {
            self.depth_replaced_entries[index] = new_entry;
        }
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

        let (found, score, m) = tt.get_entry(hash, -1000, 1000, 5, 0);
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

        let (_, score, m) = tt.get_entry(hash, -1000, 1000, 1, 0);
        assert_eq!(score, 100);
        assert_eq!(m, Some(m1));

        tt.submit_entry(hash, 300, 10, m2, TranspositionEntryType::Exact); // Should overwrite
        let (_, score, m) = tt.get_entry(hash, -1000, 1000, 1, 0);
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
    fn test_tt_resize() {
        let mut tt = TranspositionTable::new(1024);
        assert_eq!(tt.capacity, 1024);

        tt.resize(1);
        assert_eq!(tt.capacity, 32768);

        tt.resize(64);
        assert_eq!(tt.capacity, 2097152);

        tt.resize(128);
        assert_eq!(tt.capacity, 4194304);
    }
}
