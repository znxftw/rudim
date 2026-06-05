use crate::board::state::BoardState;
use crate::common::constants::{ASPIRATION_WINDOW_MARGIN, MAX_CENTIPAWN_EVAL, MAX_PLY};
use crate::common::moves::Move;
use crate::search::negamax;
use crate::search::pv_table::PvTable;
use crate::search::search_state::SearchState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

pub fn search(
    board_state: &mut BoardState,
    depth: u8,
    cancellation_token: &AtomicBool,
    debug_mode: &mut bool,
    search_state: &mut SearchState,
) {
    search_state.reset_search();

    let mut previous_pv = Vec::new();
    let mut pv_table = PvTable::new();

    let mut last_score: i16 = 0;

    for current_depth in 1..=depth {
        let timer = Instant::now();

        let start_nodes = search_state.nodes;

        // Aspiration Windows
        let mut alpha = i16::MIN + 1;
        let mut beta = i16::MAX - 1;

        if current_depth > 1 {
            alpha = last_score
                .saturating_sub(ASPIRATION_WINDOW_MARGIN)
                .max(i16::MIN + 1);
            beta = last_score
                .saturating_add(ASPIRATION_WINDOW_MARGIN)
                .min(i16::MAX - 1);
        }

        let mut current_score;

        loop {
            current_score = negamax::search(
                board_state,
                current_depth,
                alpha,
                beta,
                cancellation_token,
                &previous_pv,
                &mut pv_table,
                search_state,
            );

            if cancellation_token.load(Ordering::Relaxed) {
                break;
            }

            // TODO: Gradually expand window?
            if current_score <= alpha {
                alpha = i16::MIN + 1;
            } else if current_score >= beta {
                beta = i16::MAX - 1;
            } else {
                break;
            }
        }

        last_score = current_score;
        search_state.score = current_score;

        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        previous_pv = pv_table.line().to_vec();
        search_state.best_move = previous_pv.first().copied().unwrap_or(Move::NO_MOVE);

        let time_ms = timer.elapsed().as_millis().max(1) as f64;
        let nodes_traversed_now = search_state.nodes - start_nodes;
        let nps = (search_state.nodes as f64 / time_ms * 1000.0) as i32;

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
            let score_str = format_score(search_state.score);
            println!(
                "info depth {} score {} nodes {} time {} nps {} pv {}",
                current_depth, score_str, nodes_traversed_now, time_ms, nps, pv_string
            );
        }
    }
}

pub fn format_score(score: i16) -> String {
    let score_abs = (score as i32).abs();
    if (MAX_CENTIPAWN_EVAL as i32 - score_abs) <= MAX_PLY as i32 {
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
        assert_eq!(format_score(MAX_CENTIPAWN_EVAL - 1), "mate 1");
        assert_eq!(format_score(MAX_CENTIPAWN_EVAL - 3), "mate 2");
    }
}
