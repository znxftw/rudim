use crate::board::state::BoardState;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EvalContext {
    pub pst_midgame_score: i32,
    pub pst_endgame_score: i32,
}

impl EvalContext {
    pub fn from_board(board_state: &BoardState) -> Self {
        Self {
            pst_midgame_score: board_state.pst_midgame_score,
            pst_endgame_score: board_state.pst_endgame_score,
        }
    }
}

pub trait Evaluation {
    fn evaluate(board_state: &BoardState, eval_context: &EvalContext) -> i32;
}

pub fn evaluate<E: Evaluation>(board_state: &BoardState) -> i32 {
    let eval_context = EvalContext::from_board(board_state);
    E::evaluate(board_state, &eval_context)
}
