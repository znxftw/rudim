use crate::board::state::BoardState;
use crate::common::moves::Move;
use crate::common::tt;
use crate::search::{negamax, quiescence};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct IterativeState {
    best_move: Move,
    score: i32,
    nodes: i32,
}

static STATE: LazyLock<Mutex<IterativeState>> = LazyLock::new(|| {
    Mutex::new(IterativeState {
        best_move: Move::NO_MOVE,
        score: 0,
        nodes: 0,
    })
});

pub fn best_move() -> Move {
    STATE.lock().unwrap().best_move
}

pub fn score() -> i32 {
    STATE.lock().unwrap().score
}

pub fn nodes() -> i32 {
    STATE.lock().unwrap().nodes
}

pub fn reset_state() {
    let mut state = STATE.lock().unwrap();
    state.best_move = Move::NO_MOVE;
    state.score = 0;
    state.nodes = 0;
}

pub fn search(
    board_state: &mut BoardState,
    depth: u8,
    cancellation_token: &AtomicBool,
    debug_mode: &mut bool,
) {
    {
        let mut state = STATE.lock().unwrap();
        state.best_move = Move::NO_MOVE;
        state.score = 0;
        state.nodes = 0;
    }

    for current_depth in 1..=depth {
        let timer = Instant::now();

        let current_score = negamax::search(board_state, current_depth, cancellation_token);

        {
            let mut state = STATE.lock().unwrap();
            state.score = current_score;
        }

        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        {
            let mut state = STATE.lock().unwrap();
            state.best_move = board_state.best_move;
            let nodes_traversed = negamax::nodes() + quiescence::nodes();
            state.nodes += nodes_traversed;
        }

        let time_ms = timer.elapsed().as_millis().max(1) as f64;
        let (nodes_total, score_now, nodes_traversed_now) = {
            let state = STATE.lock().unwrap();
            (
                state.nodes,
                state.score,
                negamax::nodes() + quiescence::nodes(),
            )
        };
        let nps = (nodes_total as f64 / time_ms * 1000.0) as i32;

        let pv = {
            let table = tt::TT.lock().unwrap();
            table.collect_principal_variation(board_state)
        };
        let pv_string = pv
            .iter()
            .map(|m| {
                let promotion = m
                    .promotion_char()
                    .map(|c| c.to_string())
                    .unwrap_or_else(String::new);
                format!("{}{}{}", m.source, m.target, promotion)
            })
            .collect::<Vec<String>>()
            .join(" ");

        if *debug_mode {
            println!(
                "info depth {} score cp {} nodes {} time {} nps {} pv {}",
                current_depth, score_now, nodes_traversed_now, time_ms, nps, pv_string
            );
        }
    }
}
