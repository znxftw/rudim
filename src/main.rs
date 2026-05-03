use rudim::bitboard::lookups;
use rudim::uci;

fn main() {
    lookups::init();
    uci::cli::run();
}
