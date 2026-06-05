pub mod bitboard;
pub mod board;
pub mod common;
pub mod datagen;
pub mod engine;
pub mod eval;
pub mod perft;
pub mod search;
pub mod train;
pub mod uci;

use common::zobrist;

pub fn init() {
    zobrist::init();
}
