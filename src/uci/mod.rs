pub mod cli;
mod commands;
pub mod time_management;

use crate::board::state::BoardState;
use crate::common::helpers::STARTING_FEN;
use crate::common::moves::Move;
use crate::engine;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, LazyLock, Mutex};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SearchState {
    pub best_move: Move,
}

pub(crate) static SEARCH_STATE: LazyLock<Mutex<SearchState>> = LazyLock::new(|| {
    Mutex::new(SearchState {
        best_move: Move::NO_MOVE,
    })
});

pub fn run(_parameters: &[&str]) {
    let mut client = UciClient::new();
    client.run();
}

pub(crate) struct UciClient {
    pub board: Arc<Mutex<BoardState>>,
    pub debug_mode: Arc<AtomicBool>,
    pub current_search: Option<Arc<AtomicBool>>,
}

impl UciClient {
    fn new() -> Self {
        Self {
            board: Arc::new(Mutex::new(BoardState::parse_fen(STARTING_FEN))),
            debug_mode: Arc::new(AtomicBool::new(true)),
            current_search: None,
        }
    }

    fn run(&mut self) {
        self.write_id();

        let stdin = std::io::stdin();
        loop {
            let mut line = String::new();
            if stdin.read_line(&mut line).is_err() {
                continue;
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            let command = parts[0];
            let parameters = &parts[1..];

            if command == "quit" {
                std::process::exit(0);
            }

            match command {
                "isready" => self.run_isready(parameters),
                "position" => self.run_position(parameters),
                "go" => self.run_go(parameters),
                "stop" => self.run_stop(parameters),
                "ucinewgame" => self.run_ucinewgame(parameters),
                "debug" => self.run_debug(parameters),
                _ => cli::write_line(&format!("Unknown command {command}")),
            }
        }
    }

    fn write_id(&self) {
        cli::write_line(&format!("id name Rudim {}", env!("CARGO_PKG_VERSION")));
        cli::write_line("id author Vishnu B");
        cli::write_line("uciok");
    }
}

pub(crate) fn output_best_move(move_obj: Move) {
    let promotion = move_obj
        .promotion_char()
        .map(|c| c.to_string())
        .unwrap_or_default();
    cli::write_line(&format!(
        "bestmove {}{}{}",
        move_obj.source, move_obj.target, promotion
    ));
}

pub(crate) fn get_parameter(name: &str, parameters: &[&str], fallback: i32) -> i32 {
    for i in 0..parameters.len() {
        if parameters[i] == name
            && i + 1 < parameters.len()
            && let Ok(value) = parameters[i + 1].parse::<i32>()
        {
            return value;
        }
    }
    fallback
}

pub(crate) fn has_flag(name: &str, parameters: &[&str]) -> bool {
    parameters.contains(&name)
}

pub(crate) fn reset_global() {
    engine::reset();
}

pub(crate) fn set_ready() {
    engine::set_ready();
}

pub(crate) fn is_ready() -> bool {
    engine::is_ready()
}
