use crate::board::state::BoardState;
use crate::common::moves::Move;
use crate::search::pv_table::PvTable;
use crate::search::{negamax, quiescence};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct IterativeState {
    best_move: Move,
    score: i16,
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

pub fn score() -> i16 {
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

    let mut previous_pv = Vec::new();
    let mut pv_table = PvTable::new();

    for current_depth in 1..=depth {
        let timer = Instant::now();

        let current_score = negamax::search(
            board_state,
            current_depth,
            cancellation_token,
            &previous_pv,
            &mut pv_table,
        );

        {
            let mut state = STATE.lock().unwrap();
            state.score = current_score;
        }

        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        previous_pv = pv_table.line().to_vec();

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

        let pv_string = previous_pv
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
            let score_str = format_score(score_now);
            println!(
                "info depth {} score {} nodes {} time {} nps {} pv {}",
                current_depth, score_str, nodes_traversed_now, time_ms, nps, pv_string
            );
        }
    }
}

pub fn format_score(score: i16) -> String {
    let score_abs = (score as i32).abs();
    if (crate::common::constants::MAX_CENTIPAWN_EVAL as i32 - score_abs)
        <= crate::common::constants::MAX_PLY as i32
    {
        let d = crate::common::constants::MAX_CENTIPAWN_EVAL as i32 - score_abs;
        let y = (d + 1) / 2;
        let sign = if score < 0 { -1 } else { 1 };
        format!("mate {}", y * sign)
    } else {
        format!("cp {}", score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_score() {
        assert_eq!(format_score(100), "cp 100");
        assert_eq!(format_score(-500), "cp -500");
        assert_eq!(
            format_score(crate::common::constants::MAX_CENTIPAWN_EVAL - 1),
            "mate 1"
        );
        assert_eq!(
            format_score(crate::common::constants::MAX_CENTIPAWN_EVAL - 3),
            "mate 2"
        );
    }
}
