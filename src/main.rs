use rudim::bitboard::magics;
use rudim::perft;
use rudim::uci;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("--generate-magics") => {
            magics::generate_all_magic_numbers();
        }
        Some("--perft") => {
            rudim::init();
            perft::run_cli();
        }
        Some("--profile") => {
            // Intended to be used when profiling as reqd to debug CPU usage
            run_searches();
        }
        _ => {
            rudim::init();
            uci::cli::run();
        }
    }
}

fn run_searches() {
    const PROFILE_DEPTH: i32 = 13;

    rudim::init();

    let positions = [
        ("Starting Position", rudim::common::helpers::STARTING_FEN),
        ("Kiwi Pete", rudim::common::helpers::KIWI_PETE_FEN),
        ("Endgame", rudim::common::helpers::ENDGAME_FEN),
        (
            "Advanced Position",
            rudim::common::helpers::ADVANCED_MOVE_FEN,
        ),
    ];

    let cancellation_token = std::sync::atomic::AtomicBool::new(false);
    let mut debug_mode = true;

    for (name, fen) in positions {
        println!("\nProfiling Position: {}", name);
        println!("FEN: {}", fen);

        // Reset engine
        {
            let mut tt = rudim::common::tt::TT.lock().unwrap();
            tt.clear();
        }
        rudim::eval::move_ordering::reset_move_heuristic();

        let mut board = rudim::board::state::BoardState::parse_fen(fen);
        let start_time = std::time::Instant::now();
        let best_move = board.find_best_move(PROFILE_DEPTH, &cancellation_token, &mut debug_mode);
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
