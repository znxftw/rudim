use crate::uci::{UciClient, cli};

impl UciClient {
    pub(crate) fn run_isready(&mut self, _parameters: &[&str]) {
        if !self.is_ready {
            self.is_ready = true;
        }
        cli::write_line("readyok");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize_engine_when_not_ready() {
        let mut uci_client = UciClient::new();
        uci_client.is_ready = false;

        uci_client.run_isready(&[]);

        assert!(uci_client.is_ready);
    }

    #[test]
    fn should_still_respond_when_already_ready() {
        let mut uci_client = UciClient::new();
        uci_client.is_ready = true;

        uci_client.run_isready(&[]);

        assert!(uci_client.is_ready);
    }
}
