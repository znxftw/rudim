use crate::common::constants;
use crate::common::moves::Move;
use crate::common::side::Side;
use crate::uci::{
    SEARCH_STATE, UciClient, get_parameter, has_flag, output_best_move, time_management,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

impl UciClient {
    pub(crate) fn run_go(&mut self, parameters: &[&str]) {
        if let Some(cancel) = &self.current_search {
            cancel.store(true, Ordering::Relaxed);
        }

        let cancel_token = Arc::new(AtomicBool::new(false));
        self.current_search = Some(Arc::clone(&cancel_token));

        {
            let mut state = SEARCH_STATE.lock().unwrap();
            state.best_move = Move::NO_MOVE;
        }

        let depth = get_parameter("depth", parameters, 8);
        let winc = get_parameter("winc", parameters, -1);
        let binc = get_parameter("binc", parameters, -1);
        let wtime = get_parameter("wtime", parameters, -1);
        let btime = get_parameter("btime", parameters, -1);
        let movetime = get_parameter("movetime", parameters, -1);
        let infinite = has_flag("infinite", parameters);

        let (clock, increment) = {
            let board = self.board.lock().unwrap();
            if board.side_to_move == Side::White {
                (wtime, winc)
            } else {
                (btime, binc)
            }
        };

        let allotted_time = if movetime == -1 {
            if clock == -1 {
                -1
            } else {
                time_management::calculate_move_time(clock, increment)
            }
        } else {
            movetime
        };

        if infinite {
            // TODO: implement infinite
            return;
        }

        let board = Arc::clone(&self.board);
        let debug = Arc::clone(&self.debug_mode);
        let cancel_for_search = Arc::clone(&cancel_token);

        if allotted_time != -1 {
            let cancel_for_timer = Arc::clone(&cancel_token);
            // TODO: learn move / closures / threads
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(allotted_time as u64));
                cancel_for_timer.store(true, Ordering::Relaxed);
            });
        }

        let search_depth = if allotted_time == -1 {
            depth
        } else {
            constants::MAX_SEARCH_DEPTH as i32
        };

        thread::spawn(move || {
            let mut board = board.lock().unwrap();
            let mut debug_mode = debug.load(Ordering::Relaxed);
            let best_move = board.find_best_move(search_depth, &cancel_for_search, &mut debug_mode);
            debug.store(debug_mode, Ordering::Relaxed);

            {
                let mut state = SEARCH_STATE.lock().unwrap();
                state.best_move = best_move;
            }

            output_best_move(best_move);
        });
    }
}
