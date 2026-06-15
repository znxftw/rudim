use crate::board::state::BoardState;
use crate::common::constants;
use crate::common::moves::Move;
use crate::common::piece::Piece;
use crate::common::tt::{self, TranspositionEntryType};
use crate::eval::evaluate;
use crate::search::move_picker::MovePicker;
use crate::search::pv_table::PvTable;
use crate::search::search_state::SearchState;
use crate::search::{lmr, nmp, quiescence};
use std::sync::atomic::{AtomicBool, Ordering};

#[allow(clippy::too_many_arguments)]
pub fn search(
    board_state: &mut BoardState,
    depth: u8,
    alpha: i16,
    beta: i16,
    cancellation_token: &AtomicBool,
    previous_pv: &[Move],
    pv_table: &mut PvTable,
    search_state: &mut SearchState,
) -> i16 {
    let mut ctx = SearchContext {
        allow_null_move: true,
        on_pv_path: true,
        previous_pv,
        pv_table,
        cancellation_token,
        search_state,
    };

    search_internal(board_state, depth, 0, alpha, beta, None, &mut ctx)
}

fn search_internal(
    board_state: &mut BoardState,
    depth: u8,
    ply: u8,
    mut alpha: i16,
    beta: i16,
    previous_move: Option<Move>,
    ctx: &mut SearchContext,
) -> i16 {
    if ctx.cancellation_token.load(Ordering::Relaxed) {
        return 0;
    }

    ctx.pv_table.clear(ply as usize);

    let mut best_move = Move::NO_MOVE;
    let mut best_score = -constants::MAX_CENTIPAWN_EVAL;

    let is_pv_node = beta > 1 + alpha;
    let in_check = board_state.is_in_check(board_state.side_to_move);

    ctx.search_state.nodes += 1;

    if board_state.is_draw() {
        return 0;
    }

    if ply as usize >= constants::MAX_PLY {
        return quiescence::search(
            board_state,
            alpha,
            beta,
            ctx.cancellation_token,
            ctx.search_state,
        );
    }

    let (has_value, tt_score, tt_best) =
        ctx.search_state
            .tt
            .get_entry(board_state.board_hash, alpha, beta, depth, ply);

    // TODO: determine improvement for not returning in PV nodes
    if has_value && !is_pv_node {
        return tt_score;
    }

    if depth == 0 {
        return quiescence::search(
            board_state,
            alpha,
            beta,
            ctx.cancellation_token,
            ctx.search_state,
        );
    }

    // PRUNE: Reverse Futility Pruning
    // TODO: tune conditions
    let mut static_eval = 0;
    let has_static_eval = !is_pv_node && !in_check;

    if has_static_eval {
        static_eval = evaluate(board_state);
        // TODO: tune
        let margin = 150 * depth as i16;
        if static_eval.saturating_sub(margin) >= beta {
            return static_eval;
        }
    }

    // PRUNE: Null Move Pruning
    if nmp::can_prune(
        is_pv_node,
        board_state,
        ctx.allow_null_move,
        depth,
        in_check,
    ) {
        board_state.make_null_move();
        let reduction = nmp::get_reduction(depth);
        let score = -search_internal(
            board_state,
            depth.saturating_sub(reduction),
            ply + 1,
            -beta,
            -beta + 1,
            None,
            &mut SearchContext {
                allow_null_move: false,
                on_pv_path: false,
                previous_pv: ctx.previous_pv,
                pv_table: &mut *ctx.pv_table,
                cancellation_token: ctx.cancellation_token,
                search_state: ctx.search_state,
            },
        );
        board_state.undo_null_move();

        if ctx.cancellation_token.load(Ordering::Relaxed) {
            return 0;
        }

        if score >= beta {
            ctx.search_state.tt.submit_entry(
                board_state.board_hash,
                tt::TranspositionTable::adjust_score(score, ply as i32),
                depth,
                Move::NO_MOVE,
                TranspositionEntryType::Beta,
            );
            return score;
        }
    }

    let pv_move = if ctx.on_pv_path && (ply as usize) < ctx.previous_pv.len() {
        Some(ctx.previous_pv[ply as usize])
    } else {
        None
    };

    let mut found_pv = false;
    let mut entry_type = TranspositionEntryType::Alpha;

    let mut move_picker = MovePicker::new(pv_move, tt_best, previous_move, ply as usize);
    let mut number_of_legal_moves = 0;
    let mut has_legal_moves = false;

    while let Some(move_obj) = move_picker.next(board_state, &ctx.search_state.move_ordering) {
        if ctx.cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        board_state.make_move(move_obj);
        if board_state.is_in_check(board_state.side_to_move.other()) {
            board_state.unmake_move(move_obj);
            continue;
        }

        let gives_check = board_state.is_in_check(board_state.side_to_move);
        let is_tactical = move_obj.is_capture() || move_obj.is_promotion() || gives_check;

        has_legal_moves = true;

        // PRUNE: Futility Pruning
        if has_static_eval
            && depth < 3
            && !is_tactical
            // TODO: tune
            && static_eval.saturating_add(150 * depth as i16) <= alpha
        {
            board_state.unmake_move(move_obj);
            continue;
        }

        let needs_lmr = lmr::needs_reduction(depth, number_of_legal_moves, is_tactical, in_check);

        let mut score;
        let next_on_pv = ctx.on_pv_path && Some(move_obj) == pv_move;

        // REDUCTION: Late Move Reductions
        if needs_lmr {
            let reduction = lmr::get_reduction(depth, number_of_legal_moves);
            score = -search_internal(
                board_state,
                depth.saturating_sub(1 + reduction),
                ply + 1,
                -alpha - 1,
                -alpha,
                Some(move_obj),
                &mut SearchContext {
                    allow_null_move: ctx.allow_null_move,
                    on_pv_path: false,
                    previous_pv: ctx.previous_pv,
                    pv_table: &mut *ctx.pv_table,
                    cancellation_token: ctx.cancellation_token,
                    search_state: ctx.search_state,
                },
            );

            if score > alpha {
                let mut child_ctx = SearchContext {
                    allow_null_move: ctx.allow_null_move,
                    on_pv_path: next_on_pv,
                    previous_pv: ctx.previous_pv,
                    pv_table: &mut *ctx.pv_table,
                    cancellation_token: ctx.cancellation_token,
                    search_state: ctx.search_state,
                };
                score = search_deeper(
                    board_state,
                    depth,
                    ply,
                    alpha,
                    beta,
                    found_pv,
                    Some(move_obj),
                    &mut child_ctx,
                );
            }
        } else {
            let mut child_ctx = SearchContext {
                allow_null_move: ctx.allow_null_move,
                on_pv_path: next_on_pv,
                previous_pv: ctx.previous_pv,
                pv_table: &mut *ctx.pv_table,
                cancellation_token: ctx.cancellation_token,
                search_state: ctx.search_state,
            };
            score = search_deeper(
                board_state,
                depth,
                ply,
                alpha,
                beta,
                found_pv,
                Some(move_obj),
                &mut child_ctx,
            );
        }

        number_of_legal_moves += 1;

        board_state.unmake_move(move_obj);

        if ctx.cancellation_token.load(Ordering::Relaxed) {
            return 0;
        }

        if score > best_score {
            best_score = score;

            if score > alpha {
                alpha_update(
                    score,
                    move_obj,
                    board_state,
                    depth,
                    &mut alpha,
                    &mut best_move,
                    ctx.search_state,
                );
                entry_type = TranspositionEntryType::Exact;
                found_pv = true;

                ctx.pv_table.update(ply as usize, move_obj);
            }
        }

        if score >= beta {
            return beta_cutoff(
                score,
                move_obj,
                ply as usize,
                board_state,
                depth,
                previous_move,
                ctx.search_state,
            );
        }
    }

    if !has_legal_moves {
        if in_check {
            return -constants::MAX_CENTIPAWN_EVAL + ply as i16;
        }
        return 0;
    }

    if !ctx.cancellation_token.load(Ordering::Relaxed) {
        ctx.search_state.tt.submit_entry(
            board_state.board_hash,
            tt::TranspositionTable::adjust_score(best_score, ply as i32),
            depth,
            best_move,
            entry_type,
        );
    }

    best_score
}

#[allow(clippy::too_many_arguments)]
fn search_deeper(
    board_state: &mut BoardState,
    depth: u8,
    ply: u8,
    alpha: i16,
    beta: i16,
    found_pv: bool,
    previous_move: Option<Move>,
    ctx: &mut SearchContext,
) -> i16 {
    if found_pv {
        principal_variation_search(board_state, depth, ply, alpha, beta, previous_move, ctx)
    } else {
        -search_internal(
            board_state,
            depth.saturating_sub(1),
            ply + 1,
            -beta,
            -alpha,
            previous_move,
            &mut SearchContext {
                allow_null_move: ctx.allow_null_move,
                on_pv_path: ctx.on_pv_path,
                previous_pv: ctx.previous_pv,
                pv_table: &mut *ctx.pv_table,
                cancellation_token: ctx.cancellation_token,
                search_state: ctx.search_state,
            },
        )
    }
}

fn principal_variation_search(
    board_state: &mut BoardState,
    depth: u8,
    ply: u8,
    alpha: i16,
    beta: i16,
    previous_move: Option<Move>,
    ctx: &mut SearchContext,
) -> i16 {
    let mut score = -search_internal(
        board_state,
        depth.saturating_sub(1),
        ply + 1,
        -alpha - 1,
        -alpha,
        previous_move,
        &mut SearchContext {
            allow_null_move: ctx.allow_null_move,
            on_pv_path: ctx.on_pv_path,
            previous_pv: ctx.previous_pv,
            pv_table: &mut *ctx.pv_table,
            cancellation_token: ctx.cancellation_token,
            search_state: ctx.search_state,
        },
    );
    if score > alpha && score < beta {
        score = -search_internal(
            board_state,
            depth.saturating_sub(1),
            ply + 1,
            -beta,
            -alpha,
            previous_move,
            &mut SearchContext {
                allow_null_move: ctx.allow_null_move,
                on_pv_path: ctx.on_pv_path,
                previous_pv: ctx.previous_pv,
                pv_table: &mut *ctx.pv_table,
                cancellation_token: ctx.cancellation_token,
                search_state: ctx.search_state,
            },
        );
    }
    score
}

#[inline(always)]
fn alpha_update(
    score: i16,
    move_obj: Move,
    board_state: &mut BoardState,
    depth: u8,
    alpha: &mut i16,
    best_move: &mut Move,
    search_state: &mut SearchState,
) {
    if !move_obj.is_capture() {
        let piece = board_state.get_piece_on(move_obj.source) as usize;
        search_state
            .move_ordering
            .add_history_move(piece, move_obj, depth);
    }
    *alpha = score;
    *best_move = move_obj;
}

fn beta_cutoff(
    score: i16,
    move_obj: Move,
    ply: usize,
    board_state: &BoardState,
    depth: u8,
    previous_move: Option<Move>,
    search_state: &mut SearchState,
) -> i16 {
    search_state.tt.submit_entry(
        board_state.board_hash,
        tt::TranspositionTable::adjust_score(score, ply as i32),
        depth,
        move_obj,
        TranspositionEntryType::Beta,
    );

    if !move_obj.is_capture() {
        search_state.move_ordering.add_killer_move(move_obj, ply);

        if let Some(prev_mv) = previous_move {
            let prev_side = board_state.side_to_move.other();
            let prev_piece = board_state.piece_mapping[prev_mv.target as usize];
            if prev_piece != Piece::None {
                search_state.move_ordering.add_counter_move(
                    prev_side,
                    prev_piece,
                    prev_mv.target,
                    move_obj,
                );
            }
        }
    }

    score
}

pub struct SearchContext<'a> {
    pub allow_null_move: bool,
    pub on_pv_path: bool,
    pub previous_pv: &'a [Move],
    pub pv_table: &'a mut PvTable,
    pub cancellation_token: &'a AtomicBool,
    pub search_state: &'a mut SearchState,
}
