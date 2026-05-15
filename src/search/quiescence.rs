use crate::board::state::BoardState;
use crate::common::constants::MAX_PLY;
use crate::eval;
use crate::eval::move_ordering;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static NODES: AtomicI32 = AtomicI32::new(0);

pub fn search(
    board_state: &mut BoardState,
    mut alpha: i32,
    beta: i32,
    cancellation_token: &AtomicBool,
) -> i32 {
    NODES.fetch_add(1, Ordering::Relaxed);

    if board_state.is_draw() {
        return 0;
    }

    let eval = eval::evaluate(board_state);

    if eval >= beta {
        return beta;
    }
    if eval > alpha {
        alpha = eval;
    }

    board_state.generate_moves();

    let mut moves = board_state.moves.clone();
    move_ordering::populate_move_scores(&mut moves, board_state, MAX_PLY - 1);

    for i in 0..moves.len() {
        move_ordering::MoveOrdering::sort_next_best_move(&mut moves, i);
        let move_obj = moves[i];

        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        if !move_obj.is_capture() {
            break; // once sorted, remaining moves are quiet
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
