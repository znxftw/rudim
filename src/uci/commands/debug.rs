use crate::uci::UciClient;
use std::sync::atomic::Ordering;

impl UciClient {
    pub(crate) fn run_debug(&mut self, parameters: &[&str]) {
        if let Some(mode) = parameters.first() {
            match mode.to_ascii_lowercase().as_str() {
                "on" => self.debug_mode.store(true, Ordering::Relaxed),
                "off" => self.debug_mode.store(false, Ordering::Relaxed),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_debug_mode_on() {
        let mut uci_client = UciClient::new();
        uci_client.run_debug(&["on"]);
        assert!(uci_client.debug_mode.load(Ordering::Relaxed));
    }

    #[test]
    fn should_set_debug_mode_off() {
        let mut uci_client = UciClient::new();
        uci_client.run_debug(&["off"]);
        assert!(!uci_client.debug_mode.load(Ordering::Relaxed));
    }

    #[test]
    fn should_not_change_debug_mode_with_invalid_parameter() {
        let mut uci_client = UciClient::new();
        let initial = uci_client.debug_mode.load(Ordering::Relaxed);

        uci_client.run_debug(&["invalid"]);

        assert_eq!(initial, uci_client.debug_mode.load(Ordering::Relaxed));
    }
}
