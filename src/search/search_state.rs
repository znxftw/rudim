use crate::common::moves::Move;
use crate::common::tt::TranspositionTable;
use crate::eval::move_ordering::MoveOrdering;

pub struct SearchState {
    pub best_move: Move,
    pub score: i16,
    pub nodes: i32,
    pub move_ordering: MoveOrdering,
    pub tt: TranspositionTable,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            best_move: Move::NO_MOVE,
            score: 0,
            nodes: 0,
            move_ordering: MoveOrdering::new(),
            tt: TranspositionTable::new(TranspositionTable::DEFAULT_CAPACITY),
        }
    }

    pub fn reset_search(&mut self) {
        self.best_move = Move::NO_MOVE;
        self.score = 0;
        self.nodes = 0;
    }

    pub fn reset_heuristics(&mut self) {
        self.move_ordering.reset();
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
