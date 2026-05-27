use crate::board::state::BoardState;
use crate::common::helpers::{ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use crate::common::move_list::MoveList;

#[derive(Clone, Copy)]
pub struct PerftData {
    pub depth: u8,
    pub expected_nodes: u64,
    pub fen: &'static str,
}

const PERFT_DATA: [PerftData; 19] = [
    PerftData {
        depth: 0,
        expected_nodes: 1,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 1,
        expected_nodes: 20,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 2,
        expected_nodes: 400,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 3,
        expected_nodes: 8_902,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 4,
        expected_nodes: 197_281,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 5,
        expected_nodes: 4_865_609,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 6,
        expected_nodes: 119_060_324,
        fen: STARTING_FEN,
    },
    PerftData {
        depth: 1,
        expected_nodes: 48,
        fen: KIWI_PETE_FEN,
    },
    PerftData {
        depth: 2,
        expected_nodes: 2_039,
        fen: KIWI_PETE_FEN,
    },
    PerftData {
        depth: 3,
        expected_nodes: 97_862,
        fen: KIWI_PETE_FEN,
    },
    PerftData {
        depth: 4,
        expected_nodes: 4_085_603,
        fen: KIWI_PETE_FEN,
    },
    PerftData {
        depth: 5,
        expected_nodes: 193_690_690,
        fen: KIWI_PETE_FEN,
    },
    PerftData {
        depth: 1,
        expected_nodes: 14,
        fen: ENDGAME_FEN,
    },
    PerftData {
        depth: 2,
        expected_nodes: 191,
        fen: ENDGAME_FEN,
    },
    PerftData {
        depth: 3,
        expected_nodes: 2_812,
        fen: ENDGAME_FEN,
    },
    PerftData {
        depth: 4,
        expected_nodes: 43_238,
        fen: ENDGAME_FEN,
    },
    PerftData {
        depth: 5,
        expected_nodes: 674_624,
        fen: ENDGAME_FEN,
    },
    PerftData {
        depth: 6,
        expected_nodes: 11_030_083,
        fen: ENDGAME_FEN,
    },
    PerftData {
        depth: 7,
        expected_nodes: 178_633_661,
        fen: ENDGAME_FEN,
    },
];

pub fn traverse(board_state: &mut BoardState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let mut moves = MoveList::new();
    board_state.generate_moves(&mut moves);

    for move_ in moves.iter() {
        board_state.make_move(move_.mv);
        if !board_state.is_in_check(board_state.side_to_move.other()) {
            nodes += traverse(board_state, depth - 1);
        }
        board_state.unmake_move(move_.mv);
    }

    nodes
}

pub fn perft_test(depth: u8, expected_nodes: u64, fen: &str) {
    let mut board_state = BoardState::parse_fen(fen);
    let start = std::time::Instant::now();
    let nodes = traverse(&mut board_state, depth);
    let duration = start.elapsed();

    if nodes != expected_nodes {
        println!(
            "Perft mismatch for FEN: {}\nExpected: {}, Actual: {}",
            fen, expected_nodes, nodes
        );
        panic!("Perft failed");
    }

    println!(
        "Perft depth {} completed in {:?} ({} nodes, {} nps)",
        depth,
        duration,
        nodes,
        (nodes as f64 / duration.as_secs_f64()) as u64
    );
}

pub fn run_cli() {
    println!("Running perft suite...");
    let total_start = std::time::Instant::now();

    for test in PERFT_DATA {
        perft_test(test.depth, test.expected_nodes, test.fen);
    }

    println!("Perft suite completed in {:?}", total_start.elapsed());
}
