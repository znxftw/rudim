use crate::board::state::BoardState;
use crate::common::piece::Piece;

// TODO: tune conditions and reduction

#[inline(always)]
pub fn can_prune(
    is_pv_node: bool,
    board_state: &BoardState,
    allow_null_move: bool,
    depth: u8,
    in_check: bool,
    static_eval: i16,
    beta: i16,
) -> bool {
    let side = board_state.side_to_move;
    let has_non_pawn_material = (board_state.occupancies[side]
        ^ board_state.get_pieces(side, Piece::Pawn)
        ^ board_state.get_pieces(side, Piece::King))
    .is_not_empty();

    allow_null_move
        && !is_pv_node
        && !in_check
        && depth >= 2
        && static_eval >= beta
        && has_non_pawn_material
}

#[inline(always)]
pub fn get_reduction(depth: u8) -> u8 {
    3 + depth / 4
}
