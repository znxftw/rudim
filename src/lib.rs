pub mod bitboard;
pub mod board;
pub mod common;
pub mod engine;
pub mod eval;
pub mod perft;
pub mod search;
pub mod uci;

pub fn init() {
    bitboard::lookups::init();
    common::zobrist::init();
}
