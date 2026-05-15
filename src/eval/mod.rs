use crate::board::state::BoardState;

pub mod context;
pub mod move_ordering;
pub mod pawns;
pub mod pst;

pub type ActiveEvaluation = pst::PieceSquareTableEvaluation;

#[inline(always)]
pub fn evaluate(board_state: &BoardState) -> i32 {
    context::evaluate::<ActiveEvaluation>(board_state)
}
