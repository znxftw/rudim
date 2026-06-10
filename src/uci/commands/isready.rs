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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn should_initialize_engine_when_not_ready() {
        let mut uci_client = UciClient::new();
        reset_global();

        uci_client.run_isready(&[]);

        assert!(is_ready());
    }

    #[test]
    #[serial]
    fn should_still_respond_when_already_ready() {
        let mut uci_client = UciClient::new();
        reset_global();
        set_ready();

        uci_client.run_isready(&[]);

        assert!(is_ready());
    }
}
