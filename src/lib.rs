pub mod bitboard;
pub mod board;
pub mod common;
pub mod engine;
pub mod eval;
pub mod search;
pub mod uci;

#[cfg(feature = "train")]
pub mod datagen;
#[cfg(not(feature = "train"))]
pub mod datagen {
    pub fn run(
        _output_path: &str,
        _num_games: usize,
        _book_path: &str,
        _depth: u8,
        _threads: usize,
    ) {
        eprintln!("Error: This build of rudim was compiled without the 'train' feature.");
        std::process::exit(1);
    }
}

#[cfg(feature = "train")]
pub mod train;
#[cfg(not(feature = "train"))]
pub mod train {
    pub fn run(_custom_dataset_path: Option<&str>) {
        eprintln!("Error: This build of rudim was compiled without the 'train' feature.");
        std::process::exit(1);
    }
}

pub fn init() {}
