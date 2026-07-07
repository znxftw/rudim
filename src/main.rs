use std::env::args;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use rudim::bitboard::magics::generate_all_magic_numbers;
use rudim::board::state::BoardState;
use rudim::common::helpers::{ADVANCED_MOVE_FEN, ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use rudim::init;
use rudim::search::search_state::SearchState;
use rudim::train::run as train_run;
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
        Some("--train") => {
            init();
            let dataset_path = raw_args.get(2).map(String::as_str);
            train_run(dataset_path);
        }
        Some("--profile") => {
            // Intended to be used when profiling as reqd to debug CPU usage
            run_searches();
        }
        Some("bench") | Some("--bench") => {
            init();
            let mut search_state = SearchState::new();
            search_state.reset_search();
            let mut board = BoardState::parse_fen(STARTING_FEN);
            let cancellation_token = AtomicBool::new(false);
            let mut debug_mode = false;

            let start_time = Instant::now();
            board.find_best_move(12, &cancellation_token, &mut debug_mode, &mut search_state);
            let duration = start_time.elapsed();

            let elapsed_secs = duration.as_secs_f64();
            let nps = if elapsed_secs > 0.0 {
                (search_state.nodes as f64 / elapsed_secs) as u64
            } else {
                0
            };
            println!("{} nodes {} nps", search_state.nodes, nps);
            exit(0);
        }
        Some("datagen") | Some("--datagen") => {
            if raw_args.len() < 5 {
                eprintln!(
                    "Usage: {} datagen <output.binpack> <number_of_games> <opening_book.fen> [depth] [threads]",
                    raw_args[0]
                );
                exit(1);
            }
            init();
            let output_path = &raw_args[2];
            let num_games = match raw_args[3].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: invalid number of games");
                    exit(1);
                }
            };
            let book_path = &raw_args[4];
            let depth = if raw_args.len() > 5 {
                match raw_args[5].parse::<u8>() {
                    Ok(d) => d,
                    Err(_) => {
                        eprintln!("Error: invalid depth");
                        exit(1);
                    }
                }
            } else {
                8
            };
            let threads = if raw_args.len() > 6 {
                match raw_args[6].parse::<usize>() {
                    Ok(t) => t,
                    Err(_) => {
                        eprintln!("Error: invalid thread count");
                        exit(1);
                    }
                }
            } else {
                std::thread::available_parallelism()
                    .map(|p| p.get())
                    .unwrap_or(4)
            };
            rudim::datagen::run(output_path, num_games, book_path, depth, threads);
            exit(0);
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
