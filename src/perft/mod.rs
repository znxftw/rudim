use crate::board::state::BoardState;

pub fn traverse(board_state: &mut BoardState, depth: i32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    board_state.generate_moves();

    let moves = board_state.moves.clone();

    for move_ in moves {
        board_state.make_move(move_);
        if !board_state.is_in_check(board_state.side_to_move.other()) {
            nodes += traverse(board_state, depth - 1);
        }
        board_state.unmake_move(move_);
    }

    nodes
}

pub fn perft_test(depth: i32, expected_nodes: u64, fen: &str) {
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
