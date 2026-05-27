use crate::board::state::BoardState;
use crate::common::constants::MAX_PLY;
use crate::eval::pst::PieceSquareTableEvaluation;
use crate::search::move_picker::MovePicker;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static NODES: AtomicI32 = AtomicI32::new(0);

pub fn search(
    board_state: &mut BoardState,
    mut alpha: i16,
    beta: i16,
    cancellation_token: &AtomicBool,
) -> i16 {
    NODES.fetch_add(1, Ordering::Relaxed);

    if board_state.is_draw() {
        return 0;
    }

    let eval = PieceSquareTableEvaluation::evaluate(board_state);

    if eval >= beta {
        return beta;
    }
    if eval > alpha {
        alpha = eval;
    }

    let mut move_picker = MovePicker::new_qsearch(MAX_PLY - 1);

    while let Some(move_obj) = move_picker.next(board_state) {
        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        board_state.make_move(move_obj);
        if board_state.is_in_check(board_state.side_to_move.other()) {
            board_state.unmake_move(move_obj);
            continue;
        }

        let score = -search(board_state, -beta, -alpha, cancellation_token);
        board_state.unmake_move(move_obj);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

pub fn reset_nodes() {
    NODES.store(0, Ordering::Relaxed);
}

pub fn nodes() -> i32 {
    NODES.load(Ordering::Relaxed)
}
