use rudim::board::state::BoardState;
use rudim::common::helpers::{ADVANCED_MOVE_FEN, ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use rudim::common::move_type::MoveType;
use rudim::common::moves::Move;
use rudim::search::search_state::SearchState;
use serial_test::serial;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn find_move_from_move_list(board: &mut BoardState, expected_move: Move) -> Move {
    let mut move_list = rudim::common::move_list::MoveList::new();
    board.generate_moves(&mut move_list);

    for m in move_list.iter() {
        if m.mv.source == expected_move.source
            && m.mv.target == expected_move.target
            && (expected_move.move_type == MoveType::Quiet
                || ((m.mv.move_type.value() & !8) == expected_move.move_type.value()))
        {
            return m.mv;
        }
    }

    Move::NO_MOVE
}

fn assert_traversal(position: &str, expected_nodes: i32, expected_score: i16, depth: u8) {
    let mut board_state = BoardState::parse_fen(position);
    let cancellation_token = AtomicBool::new(false);
    let mut debug_mode = false;
    let mut search_state = SearchState::new();

    board_state.find_best_move(
        depth,
        &cancellation_token,
        &mut debug_mode,
        &mut search_state,
    );

    assert_eq!(expected_nodes, search_state.nodes);
    assert_eq!(expected_score, search_state.score);
}

fn assert_tactic_best_move(fen: &str, move_lan: &str) {
    let mut board_state = BoardState::parse_fen(fen);

    let cancellation_token = Arc::new(AtomicBool::new(false));
    let cancellation_writer = Arc::clone(&cancellation_token);
    let cancellation_worker = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        cancellation_writer.store(true, Ordering::Relaxed);
    });

    let mut debug_mode = false;
    let mut search_state = SearchState::new();
    let best_move = board_state.find_best_move(
        25,
        cancellation_token.as_ref(),
        &mut debug_mode,
        &mut search_state,
    );

    let expected_move =
        Move::parse_long_algebraic(move_lan).expect("Failed to parse expected move");
    let expected_move = find_move_from_move_list(&mut board_state, expected_move);

    cancellation_worker.join().unwrap();

    assert_eq!(expected_move, best_move);
}

macro_rules! traversal_test_case {
    ($name:ident, $fen:expr, $nodes:expr, $score:expr, $depth:expr) => {
        #[test]
        #[serial]
        fn $name() {
            assert_traversal($fen, $nodes, $score, $depth);
        }
    };
    ($name:ident, skip = $reason:literal, $fen:expr, $nodes:expr, $score:expr, $depth:expr) => {
        #[test]
        #[ignore = $reason]
        #[serial]
        fn $name() {
            assert_traversal($fen, $nodes, $score, $depth);
        }
    };
}

macro_rules! tactic_test_case {
    ($name:ident, $fen:expr, $best_move:expr) => {
        #[test]
        #[serial]
        fn $name() {
            assert_tactic_best_move($fen, $best_move);
        }
    };
    ($name:ident, skip = $reason:literal, $fen:expr, $best_move:expr) => {
        #[test]
        #[ignore = $reason]
        #[serial]
        fn $name() {
            assert_tactic_best_move($fen, $best_move);
        }
    };
}

traversal_test_case!(traversal_starting, STARTING_FEN, 876159, 26, 13);
traversal_test_case!(traversal_endgame, ENDGAME_FEN, 578955, 386, 17);
traversal_test_case!(traversal_advanced, ADVANCED_MOVE_FEN, 1557147, 3443, 16);
traversal_test_case!(traversal_kiwi_pete, KIWI_PETE_FEN, 723398, -226, 12);

tactic_test_case!(
    tactic_random_puzzle_position,
    "r4r2/pb4kp/1p4p1/1P6/2P1pRp1/P3B3/7P/5RK1 w - - 0 29",
    "f4f8"
);
tactic_test_case!(
    tactic_transposition_table_verification,
    skip = "More Depth Needed",
    "8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - -",
    "a1b1"
);
tactic_test_case!(
    tactic_zugzwang_verification_1,
    skip = "More Depth Needed",
    "8/8/1p1r1k2/p1pPN1p1/P3KnP1/1P6/8/3R4 b - - 0 1",
    "f4d5"
);
tactic_test_case!(
    tactic_zugzwang_verification_2,
    skip = "Improve NMR",
    "7k/5K2/5P1p/3p4/6P1/3p4/8/8 w - - 0 1",
    "g4g5"
);
tactic_test_case!(
    tactic_zugzwang_verification_3,
    skip = "Improve NMR",
    "8/6B1/p5p1/Pp4kp/1P5r/5P1Q/4q1PK/8 w - - 0 32",
    "h3h4"
);
tactic_test_case!(
    tactic_zugzwang_verification_4,
    skip = "Improve NMR",
    "8/8/p1p5/1p5p/1P5p/8/PPP2K1p/4R1rk w - - 0 1",
    "e1f1"
);
tactic_test_case!(
    tactic_zugzwang_verification_5,
    skip = "Improve NMR",
    "1q1k4/2Rr4/8/2Q3K1/8/8/8/8 w - - 0 1",
    "g5h6"
);
