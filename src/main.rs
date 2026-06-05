use std::env::args;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use rudim::bitboard::magics::generate_all_magic_numbers;
use rudim::board::state::BoardState;
use rudim::common::helpers::{ADVANCED_MOVE_FEN, ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use rudim::init;
use rudim::perft::run_cli;
use rudim::search::search_state::SearchState;
use rudim::train::{convert_text_to_bin, run as train_run};
use rudim::uci::cli::run as uci_run;

fn main() {
    let raw_args: Vec<String> = args().collect();

    match raw_args.get(1).map(String::as_str) {
        Some("--randomize-weights") => {
            use rudim::eval::nnue::loader::Network;
            let mut network = Network::new_boxed();
            network.randomize();
            if let Err(e) = network.save_to_file("resources/nnue.bin") {
                eprintln!("Error saving randomized weights: {}", e);
                exit(1);
            }
            println!("Successfully randomized weights and saved to resources/nnue.bin!");
        }
        Some("--generate-magics") => {
            generate_all_magic_numbers();
        }
        Some("--perft") => {
            init();
            run_cli();
        }
        Some("--train") => {
            init();
            let dataset_path = raw_args.get(2).map(String::as_str);
            let checkpoint_path = raw_args.get(3).map(String::as_str);
            train_run(dataset_path, checkpoint_path);
        }
        Some("--convert") => {
            if raw_args.len() < 4 {
                eprintln!("Usage: rudim --convert <input.txt> <output.data>");
                exit(1);
            }
            let input_path = &raw_args[2];
            let output_path = &raw_args[3];
            if let Err(e) = convert_text_to_bin(input_path, output_path) {
                eprintln!("Error converting file: {}", e);
                exit(1);
            }
        }
        Some("--profile") => {
            // Intended to be used when profiling as reqd to debug CPU usage
            run_searches();
        }
        _ => {
            init();
            uci_run();
        }
    }
}

fn run_searches() {
    const PROFILE_DEPTH: u8 = 13;

    init();

    let positions = [
        ("Starting Position", STARTING_FEN),
        ("Kiwi Pete", KIWI_PETE_FEN),
        ("Endgame", ENDGAME_FEN),
        ("Advanced Position", ADVANCED_MOVE_FEN),
    ];

    let cancellation_token = AtomicBool::new(false);
    let mut debug_mode = true;
    let mut search_state = SearchState::new();

    for (name, fen) in positions {
        println!("\nProfiling Position: {}", name);
        println!("FEN: {}", fen);

        // Reset search state
        search_state.tt.clear();
        search_state.reset_heuristics();
        search_state.reset_search();

        let mut board = BoardState::parse_fen(fen);
        let start_time = Instant::now();
        let best_move = board.find_best_move(
            PROFILE_DEPTH,
            &cancellation_token,
            &mut debug_mode,
            &mut search_state,
        );
        let duration = start_time.elapsed();

        let promo = best_move
            .promotion_char()
            .map(|c| c.to_string())
            .unwrap_or_default();
        println!(
            "Best move: {}{}{}",
            best_move.source, best_move.target, promo
        );
        println!("Time taken: {:?}", duration);
    }
}
