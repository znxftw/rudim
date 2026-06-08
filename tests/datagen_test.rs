use rudim::board::state::BoardState;
use rudim::common::move_type::MoveType;
use rudim::common::moves::Move;
use rudim::common::square::Square;
use rudim::datagen::{board_state_to_viriboard, map_rudim_move, read_metadata, run as run_datagen};
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_fen_loading_and_mapping() {
    let book_path = "tests/test_book.fen";
    let fens = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
    ];
    let mut file = File::create(book_path).unwrap();
    for fen in &fens {
        writeln!(file, "{}", fen).unwrap();
    }
    drop(file);

    let loaded = rudim::datagen::load_opening_book(book_path).unwrap();
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0], fens[0]);
    assert_eq!(loaded[1], fens[1]);

    let state = BoardState::parse_fen(fens[0]);
    let viri_board = board_state_to_viriboard(&state);

    let viri_debug = format!("{:?}", viri_board);
    assert!(viri_debug.contains("side: White"));
    assert!(viri_debug.contains("fifty_move_counter: 0"));

    let m = Move::new(Square::E2, Square::E4, MoveType::DoublePush);
    let viri_move = map_rudim_move(&m);
    assert_eq!(viri_move.from().inner(), 12); // E2 index ^ 56 in viriformat = (52 ^ 56) = 12 (E2)
    assert_eq!(viri_move.to().inner(), 28); // E4 index ^ 56 in viriformat = (36 ^ 56) = 28 (E4)

    let m_wk = Move::new(Square::E1, Square::G1, MoveType::Castle);
    let viri_wk = map_rudim_move(&m_wk);
    assert_eq!(viri_wk.from().inner(), 4); // E1 (60 ^ 56 = 4)
    assert_eq!(viri_wk.to().inner(), 7); // H1 (63 ^ 56 = 7)

    let m_wq = Move::new(Square::E1, Square::C1, MoveType::Castle);
    let viri_wq = map_rudim_move(&m_wq);
    assert_eq!(viri_wq.from().inner(), 4); // E1 (60 ^ 56 = 4)
    assert_eq!(viri_wq.to().inner(), 0); // A1 (56 ^ 56 = 0)

    let m_bk = Move::new(Square::E8, Square::G8, MoveType::Castle);
    let viri_bk = map_rudim_move(&m_bk);
    assert_eq!(viri_bk.from().inner(), 60); // E8 (4 ^ 56 = 60)
    assert_eq!(viri_bk.to().inner(), 63); // H8 (7 ^ 56 = 63)

    let m_bq = Move::new(Square::E8, Square::C8, MoveType::Castle);
    let viri_bq = map_rudim_move(&m_bq);
    assert_eq!(viri_bq.from().inner(), 60); // E8 (4 ^ 56 = 60)
    assert_eq!(viri_bq.to().inner(), 56); // A8 (0 ^ 56 = 56)

    // Cleanup
    let _ = fs::remove_file(book_path);
}

#[test]
fn test_mini_datagen_run() {
    let book_path = "tests/test_book_mini.fen";
    let mut file = File::create(book_path).unwrap();
    writeln!(
        file,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    )
    .unwrap();
    drop(file);

    let output_binpack = "tests/test_output.binpack";
    let output_metadata = format!("{}.meta", output_binpack);
    let _ = fs::remove_file(output_binpack);
    let _ = fs::remove_file(&output_metadata);

    // Run 1: generate 2 games
    run_datagen(output_binpack, 2, book_path, 2, 2);

    let metadata = fs::metadata(output_binpack).unwrap();
    assert!(metadata.len() > 0);
    assert!(metadata.len() >= 72);

    let meta = read_metadata(output_binpack);
    assert_eq!(meta.games_completed, 2);
    assert!(meta.total_positions > 0);

    // Run 2: append 3 games (total should become 5)
    run_datagen(output_binpack, 3, book_path, 2, 2);

    let meta2 = read_metadata(output_binpack);
    assert_eq!(meta2.games_completed, 5);
    assert!(meta2.total_positions > meta.total_positions);

    let _ = fs::remove_file(book_path);
    let _ = fs::remove_file(output_binpack);
    let _ = fs::remove_file(&output_metadata);
}
