pub mod cli;

pub fn run(_parameters: &[&str]) {
    // Phase 9.2 will implement full UCI client/command set.
    // For 9.1, keep command dispatch in place.
    cli::write_line("uci command is not implemented yet");
}
