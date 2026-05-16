use crate::common::move_type::MoveType;
use crate::common::moves::Move;
use crate::common::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScoredMove {
    pub mv: Move,
    pub score: i32,
}

impl ScoredMove {
    pub fn new(source: Square, target: Square, move_type: MoveType) -> Self {
        Self {
            mv: Move::new(source, target, move_type),
            score: 0,
        }
    }
}
