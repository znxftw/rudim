use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::square::Square;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScoredMove {
    pub mv: Move,
    pub score: i32,
}

pub const NO_SCORED_MOVE: ScoredMove = ScoredMove {
    mv: Move::NO_MOVE,
    score: 0,
};

impl ScoredMove {
    pub fn new(source: Square, target: Square, move_type: MoveType) -> Self {
        Self {
            mv: Move::new(source, target, move_type),
            score: 0,
        }
    }
}

pub const MAX_MOVES: usize = 218;

#[derive(Clone, Copy)]
pub struct MoveList {
    pub moves: [ScoredMove; MAX_MOVES],
    pub count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [NO_SCORED_MOVE; MAX_MOVES],
            count: 0,
        }
    }

    pub fn push(&mut self, m: ScoredMove) {
        self.moves[self.count] = m;
        self.count += 1;
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }
}

impl Deref for MoveList {
    type Target = [ScoredMove];

    fn deref(&self) -> &Self::Target {
        &self.moves[..self.count]
    }
}

impl DerefMut for MoveList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.moves[..self.count]
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}
