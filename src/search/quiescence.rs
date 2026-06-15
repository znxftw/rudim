use crate::board::state::BoardState;
use crate::common::constants::MAX_PLY;
use crate::eval::evaluate;
use crate::search::move_picker::MovePicker;
use crate::search::search_state::SearchState;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn search(
    board_state: &mut BoardState,
    mut alpha: i16,
    beta: i16,
    cancellation_token: &AtomicBool,
    search_state: &mut SearchState,
) -> i16 {
    if cancellation_token.load(Ordering::Relaxed) {
        return 0;
    }

    search_state.nodes += 1;

    if board_state.is_draw() {
        return 0;
    }

    let eval = evaluate(board_state);

    if eval >= beta {
        return beta;
    }
    if eval > alpha {
        alpha = eval;
    }

    let mut move_picker = MovePicker::new_qsearch(MAX_PLY - 1);

    while let Some(move_obj) = move_picker.next(board_state, &search_state.move_ordering) {
        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        board_state.make_move(move_obj);
        if board_state.is_in_check(board_state.side_to_move.other()) {
            board_state.unmake_move(move_obj);
            continue;
        }

        let score = -search(board_state, -beta, -alpha, cancellation_token, search_state);
        board_state.unmake_move(move_obj);

        if cancellation_token.load(Ordering::Relaxed) {
            return 0;
        }

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
