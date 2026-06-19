use crate::board::state::BoardState;
use crate::common::piece::Piece;

#[inline(always)]
pub fn can_prune(
    is_pv_node: bool,
    board_state: &BoardState,
    allow_null_move: bool,
    depth: u8,
    in_check: bool,
) -> bool {
    let side = board_state.side_to_move;
    let has_non_pawn_material = (board_state.occupancies[side]
        ^ board_state.get_pieces(side, Piece::Pawn)
        ^ board_state.get_pieces(side, Piece::King))
    .is_not_empty();

    allow_null_move && !is_pv_node && !in_check && depth >= 2 && has_non_pawn_material
}

#[inline(always)]
pub fn get_reduction(_depth: u8) -> u8 {
    // TODO: In the future, this can be tuned dynamically.
    3
}
