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
        _ => {
            rudim::init();
            uci::cli::run();
        }
    }
}
