use crate::uci::{UciClient, cli, is_ready, reset_global, set_ready};

impl UciClient {
    pub(crate) fn run_isready(&mut self, _parameters: &[&str]) {
        if !is_ready() {
            reset_global();
            set_ready();
        }
        cli::write_line("readyok");
    }
}
