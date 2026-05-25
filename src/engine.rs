use crate::common::tt;
use crate::eval::move_ordering;
use crate::search::{iterative_deepening, negamax, quiescence};
use std::sync::atomic::{AtomicBool, Ordering};

static IS_READY: AtomicBool = AtomicBool::new(false);

pub fn reset() {
    // TODO: some of these need to be reset in between searches - expose another reset_intermediate()
    IS_READY.store(false, Ordering::Relaxed);
    move_ordering::reset_move_heuristic();
    iterative_deepening::reset_state();
    negamax::reset_state();
    quiescence::reset_nodes();
    tt::TT.lock().unwrap().clear();
}

pub fn set_ready() {
    IS_READY.store(true, Ordering::Relaxed);
}

pub fn is_ready() -> bool {
    IS_READY.load(Ordering::Relaxed)
}
