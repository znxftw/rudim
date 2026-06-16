use crate::uci::UciClient;

impl UciClient {
    pub(crate) fn run_setoption(&mut self, parameters: &[&str]) {
        let mut name = String::new();
        let mut value = String::new();

        let mut i = 0;
        while i < parameters.len() {
            if parameters[i].eq_ignore_ascii_case("name") && i + 1 < parameters.len() {
                let mut name_parts = Vec::new();
                i += 1;
                while i < parameters.len() && !parameters[i].eq_ignore_ascii_case("value") {
                    name_parts.push(parameters[i]);
                    i += 1;
                }
                name = name_parts.join(" ");
                continue;
            }
            if parameters[i].eq_ignore_ascii_case("value") && i + 1 < parameters.len() {
                value = parameters[i + 1..].join(" ");
                break;
            }
            i += 1;
        }

        if name.eq_ignore_ascii_case("Hash") {
            if let Ok(mb_size) = value.parse::<usize>() {
                let mb_size = mb_size.clamp(1, 2048);
                if let Ok(mut state) = self.search_state.try_lock() {
                    state.tt.resize(mb_size);
                }
                // else, as per UCI specs, we should just ignore it (e.g. attempt to resize mid-search)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn should_resize_transposition_table() {
        let mut uci_client = UciClient::new();

        {
            let state = uci_client.search_state.lock().unwrap();
            assert_eq!(state.tt.capacity(), 2097152); // 64MB
        }

        uci_client.run_setoption(&["name", "Hash", "value", "128"]);

        {
            let state = uci_client.search_state.lock().unwrap();
            assert_eq!(state.tt.capacity(), 4194304); // 128MB
        }

        uci_client.run_setoption(&["name", "hash", "value", "1"]);

        {
            let state = uci_client.search_state.lock().unwrap();
            assert_eq!(state.tt.capacity(), 32768); // 1MB
        }
    }
}
