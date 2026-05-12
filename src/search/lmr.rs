use crate::board::state::BoardState;
use crate::common::moves::Move;

#[inline(always)]
pub fn needs_reduction(
    depth: i32,
    number_of_legal_moves: usize,
    move_obj: Move,
    board_state: &BoardState,
    is_evading_check: bool,
) -> bool {
    if depth < 3 || number_of_legal_moves < 3 || is_evading_check {
        return false;
    }

    if move_obj.is_capture() || move_obj.is_promotion() {
        return false;
    }

    if board_state.is_in_check(board_state.side_to_move) {
        return false;
    }

    true
}

#[inline(always)]
pub fn get_reduction(_depth: i32, _number_of_legal_moves: usize) -> i32 {
    1
}
