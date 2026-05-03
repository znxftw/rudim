use crate::board::state::BoardState;
use crate::uci::{UciClient, reset_global, set_ready};

impl UciClient {
    pub(crate) fn run_ucinewgame(&mut self, _parameters: &[&str]) {
        reset_global();
        *self.board.lock().unwrap() = BoardState::default();
        set_ready();
    }
}
