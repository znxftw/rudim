use crate::common::moves::Move;
use crate::uci::{SEARCH_STATE, UciClient, output_best_move};
use std::sync::atomic::Ordering;

impl UciClient {
    pub(crate) fn run_stop(&mut self, _parameters: &[&str]) {
        if let Some(cancel) = &self.current_search {
            cancel.store(true, Ordering::Relaxed);
        }

        let best = SEARCH_STATE.lock().unwrap().best_move;
        if best != Move::NO_MOVE {
            output_best_move(best);
        }
    }
}
