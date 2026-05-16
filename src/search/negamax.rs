use crate::board::state::BoardState;
use crate::common::constants;
use crate::common::tt::{self, TranspositionEntryType};
use crate::eval::move_ordering;
use crate::search::quiescence;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering};

static NODES: AtomicI32 = AtomicI32::new(0);
static SEARCH_DEPTH: AtomicU8 = AtomicU8::new(0);

pub fn nodes() -> i32 {
    NODES.load(Ordering::Relaxed)
}

pub fn reset_state() {
    NODES.store(0, Ordering::Relaxed);
    SEARCH_DEPTH.store(0, Ordering::Relaxed);
}

pub fn search(board_state: &mut BoardState, depth: u8, cancellation_token: &AtomicBool) -> i16 {
    SEARCH_DEPTH.store(depth, Ordering::Relaxed);
    NODES.store(0, Ordering::Relaxed);
    quiescence::reset_nodes();

    search_internal(
        board_state,
        depth,
        i16::MIN + 1,
        i16::MAX - 1,
        true,
        cancellation_token,
    )
}

fn search_internal(
    board_state: &mut BoardState,
    depth: u8,
    mut alpha: i16,
    beta: i16,
    allow_null_move: bool,
    cancellation_token: &AtomicBool,
) -> i16 {
    let ply = SEARCH_DEPTH.load(Ordering::Relaxed) - depth;
    let is_pv_node = beta > 1 + alpha; // beta - alpha > 1 overflow

    NODES.fetch_add(1, Ordering::Relaxed);

    if board_state.is_draw() {
        return 0;
    }

    let (has_value, tt_score, tt_best) = {
        let table = tt::TT.lock().unwrap();
        table.get_entry(board_state.board_hash, alpha, beta, depth)
    };

    if has_value {
        if let Some(best) = tt_best
            && best != crate::common::moves::Move::NO_MOVE
        {
            board_state.best_move = best;
        }
        return tt::TranspositionTable::retrieve_score(tt_score, ply as i32);
    }

    if depth == 0 {
        return quiescence::search(board_state, alpha, beta, cancellation_token);
    }

    if can_prune_null_move(is_pv_node, board_state, allow_null_move, depth) {
        board_state.make_null_move();
        let score = -search_internal(
            board_state,
            depth.saturating_sub(3),
            -beta,
            -beta + 1,
            false,
            cancellation_token,
        );
        board_state.undo_null_move();

        if score >= beta {
            let mut table = tt::TT.lock().unwrap();
            table.submit_entry(
                board_state.board_hash,
                tt::TranspositionTable::adjust_score(beta, ply as i32),
                depth,
                crate::common::moves::Move::NO_MOVE,
                TranspositionEntryType::Beta,
            );
            return beta;
        }
    }

    let mut found_pv = false;
    let mut entry_type = TranspositionEntryType::Alpha;

    board_state.generate_moves();
    populate_move_scores(board_state, ply as usize, tt_best);

    let mut number_of_legal_moves = 0;

    let mut moves = board_state.moves.clone();
    for i in 0..moves.len() {
        move_ordering::MoveOrdering::sort_next_best_move(&mut moves, i);
        let move_obj = moves[i];

        if cancellation_token.load(Ordering::Relaxed) {
            break;
        }

        board_state.make_move(move_obj);
        if board_state.is_in_check(board_state.side_to_move.other()) {
            board_state.unmake_move(move_obj);
            continue;
        }

        let needs_lmr = crate::search::lmr::needs_reduction(depth, number_of_legal_moves);

        let mut score;

        if needs_lmr {
            let reduction = crate::search::lmr::get_reduction(depth, number_of_legal_moves);
            score = -search_internal(
                board_state,
                depth.saturating_sub(1 + reduction),
                -alpha - 1,
                -alpha,
                allow_null_move,
                cancellation_token,
            );

            if score > alpha {
                score = search_deeper(
                    board_state,
                    depth,
                    alpha,
                    beta,
                    cancellation_token,
                    found_pv,
                    allow_null_move,
                );
            }
        } else {
            score = search_deeper(
                board_state,
                depth,
                alpha,
                beta,
                cancellation_token,
                found_pv,
                allow_null_move,
            );
        }

        number_of_legal_moves += 1;

        board_state.unmake_move(move_obj);

        if score >= beta {
            return beta_cutoff(beta, move_obj, ply as usize, board_state, depth);
        }

        if score > alpha {
            alpha_update(
                score,
                move_obj,
                board_state,
                depth,
                &mut alpha,
                &mut found_pv,
                &mut entry_type,
            );
        }
    }

    if number_of_legal_moves == 0 {
        if board_state.is_in_check(board_state.side_to_move) {
            return -constants::MAX_CENTIPAWN_EVAL + ply as i16;
        }
        return 0;
    }

    {
        let mut table = tt::TT.lock().unwrap();
        table.submit_entry(
            board_state.board_hash,
            tt::TranspositionTable::adjust_score(alpha, ply as i32),
            depth,
            board_state.best_move,
            entry_type,
        );
    }

    alpha
}

fn search_deeper(
    board_state: &mut BoardState,
    depth: u8,
    alpha: i16,
    beta: i16,
    cancellation_token: &AtomicBool,
    found_pv: bool,
    allow_null_move: bool,
) -> i16 {
    if found_pv {
        principal_variation_search(
            board_state,
            depth,
            alpha,
            beta,
            allow_null_move,
            cancellation_token,
        )
    } else {
        -search_internal(
            board_state,
            depth - 1,
            -beta,
            -alpha,
            allow_null_move,
            cancellation_token,
        )
    }
}

fn principal_variation_search(
    board_state: &mut BoardState,
    depth: u8,
    alpha: i16,
    beta: i16,
    allow_null_move: bool,
    cancellation_token: &AtomicBool,
) -> i16 {
    let mut score = -search_internal(
        board_state,
        depth - 1,
        -alpha - 1,
        -alpha,
        allow_null_move,
        cancellation_token,
    );
    if score > alpha && score < beta {
        score = -search_internal(
            board_state,
            depth - 1,
            -beta,
            -alpha,
            allow_null_move,
            cancellation_token,
        );
    }
    score
}

fn alpha_update(
    score: i16,
    move_obj: crate::common::moves::Move,
    board_state: &mut BoardState,
    depth: u8,
    alpha: &mut i16,
    found_pv: &mut bool,
    entry_type: &mut TranspositionEntryType,
) {
    *entry_type = TranspositionEntryType::Exact;
    if !move_obj.is_capture() {
        let piece = board_state.get_piece_on(move_obj.source) as usize;
        move_ordering::add_history_move(piece, move_obj, depth);
    }
    *alpha = score;
    board_state.best_move = move_obj;
    *found_pv = true;
}

fn beta_cutoff(
    beta: i16,
    move_obj: crate::common::moves::Move,
    ply: usize,
    board_state: &BoardState,
    depth: u8,
) -> i16 {
    {
        let mut table = tt::TT.lock().unwrap();
        table.submit_entry(
            board_state.board_hash,
            tt::TranspositionTable::adjust_score(beta, ply as i32),
            depth,
            move_obj,
            TranspositionEntryType::Beta,
        );
    }

    if !move_obj.is_capture() {
        move_ordering::add_killer_move(move_obj, ply);
    }

    beta
}

fn populate_move_scores(
    board_state: &mut BoardState,
    ply: usize,
    hash_move: Option<crate::common::moves::Move>,
) {
    // TODO: non-clone impl? mutable borrow?
    let mut moves = board_state.moves.clone();
    move_ordering::populate_move_scores(&mut moves, board_state, ply);

    if let Some(hash_move) = hash_move
        && hash_move != crate::common::moves::Move::NO_MOVE
    {
        for move_obj in &mut moves {
            if *move_obj == hash_move {
                move_ordering::populate_hash_move(move_obj);
                break;
            }
        }
    }

    board_state.moves = moves;
}

fn can_prune_null_move(
    is_pv_node: bool,
    board_state: &BoardState,
    allow_null_move: bool,
    depth: u8,
) -> bool {
    allow_null_move
        && !is_pv_node
        && !board_state.is_in_check(board_state.side_to_move)
        && depth >= 2
        && board_state.phase > crate::common::game_phase::ONLY_PAWNS
}
