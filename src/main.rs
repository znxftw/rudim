use rudim::bitboard::lookups;
use rudim::perft;
use rudim::uci;

fn main() {
    lookups::init();

    let args: Vec<String> = std::env::args().collect();

    if matches!(args.get(1).map(String::as_str), Some("--perft")) {
        perft::run_cli();
        return;
    }

    uci::cli::run();
}
