use rudim::board::state::BoardState;
use rudim::common::helpers::{ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use rudim::common::move_list::MoveList;

fn traverse(board_state: &mut BoardState, depth: u8) -> u64 {
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

fn run_perft(fen: &str, depth: u8, expected_nodes: u64) {
    let mut board_state = BoardState::parse_fen(fen);
    let start = std::time::Instant::now();
    let nodes = traverse(&mut board_state, depth);
    let duration = start.elapsed();

    assert_eq!(
        nodes, expected_nodes,
        "Perft mismatch for FEN: {}\nExpected: {}, Actual: {}",
        fen, expected_nodes, nodes
    );

    println!(
        "Perft depth {} completed in {:?} ({} nodes, {} nps)",
        depth,
        duration,
        nodes,
        (nodes as f64 / duration.as_secs_f64()) as u64
    );
}

macro_rules! perft_test_case {
    ($name:ident, $fen:expr, $depth:expr, $expected:expr) => {
        #[test]
        fn $name() {
            run_perft($fen, $depth, $expected);
        }
    };
}

// Starting Position tests
perft_test_case!(perft_starting_d0, STARTING_FEN, 0, 1);
perft_test_case!(perft_starting_d1, STARTING_FEN, 1, 20);
perft_test_case!(perft_starting_d2, STARTING_FEN, 2, 400);
perft_test_case!(perft_starting_d3, STARTING_FEN, 3, 8_902);
perft_test_case!(perft_starting_d4, STARTING_FEN, 4, 197_281);
perft_test_case!(perft_starting_d5, STARTING_FEN, 5, 4_865_609);
perft_test_case!(perft_starting_d6, STARTING_FEN, 6, 119_060_324);

// Kiwi Pete tests
perft_test_case!(perft_kiwi_pete_d1, KIWI_PETE_FEN, 1, 48);
perft_test_case!(perft_kiwi_pete_d2, KIWI_PETE_FEN, 2, 2_039);
perft_test_case!(perft_kiwi_pete_d3, KIWI_PETE_FEN, 3, 97_862);
perft_test_case!(perft_kiwi_pete_d4, KIWI_PETE_FEN, 4, 4_085_603);
perft_test_case!(perft_kiwi_pete_d5, KIWI_PETE_FEN, 5, 193_690_690);

// Endgame tests
perft_test_case!(perft_endgame_d1, ENDGAME_FEN, 1, 14);
perft_test_case!(perft_endgame_d2, ENDGAME_FEN, 2, 191);
perft_test_case!(perft_endgame_d3, ENDGAME_FEN, 3, 2_812);
perft_test_case!(perft_endgame_d4, ENDGAME_FEN, 4, 43_238);
perft_test_case!(perft_endgame_d5, ENDGAME_FEN, 5, 674_624);
perft_test_case!(perft_endgame_d6, ENDGAME_FEN, 6, 11_030_083);
perft_test_case!(perft_endgame_d7, ENDGAME_FEN, 7, 178_633_661);
