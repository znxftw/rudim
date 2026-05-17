use crate::board::state::BoardState;

#[inline(always)]
pub fn can_prune(
    is_pv_node: bool,
    board_state: &BoardState,
    allow_null_move: bool,
    depth: u8,
    in_check: bool,
) -> bool {
    allow_null_move
        && !is_pv_node
        && !in_check
        && depth >= 2
        && board_state.phase > crate::common::game_phase::ONLY_PAWNS
}

#[inline(always)]
pub fn get_reduction(_depth: u8) -> u8 {
    // TODO: In the future, this can be tuned dynamically.
    3
}
