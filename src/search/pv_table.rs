use crate::common::constants;
use crate::common::moves::Move;

#[derive(Clone)]
pub struct PvTable {
    table: Box<[[Move; constants::MAX_PLY + 1]]>,
    len: [usize; constants::MAX_PLY + 1],
}

impl PvTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line(&self) -> &[Move] {
        &self.table[0][..self.len[0]]
    }

    pub fn clear(&mut self, ply: usize) {
        if ply < constants::MAX_PLY + 1 {
            self.len[ply] = 0;
        }
    }

    pub fn update(&mut self, ply: usize, best_move: Move) {
        if ply >= constants::MAX_PLY {
            return;
        }
        self.table[ply][0] = best_move;

        let child_len = self.len[ply + 1];
        self.len[ply] = child_len + 1;

        if child_len > 0 {
            let (left, right) = self.table.split_at_mut(ply + 1);
            left[ply][1..=child_len].copy_from_slice(&right[0][0..child_len]);
        }
    }
}

impl Default for PvTable {
    fn default() -> Self {
        Self {
            table: vec![[Move::NO_MOVE; constants::MAX_PLY + 1]; constants::MAX_PLY + 1]
                .into_boxed_slice(),
            len: [0; constants::MAX_PLY + 1],
        }
    }
}
