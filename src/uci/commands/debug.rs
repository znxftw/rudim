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
