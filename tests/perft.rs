use rudim::common::helpers::{ENDGAME_FEN, KIWI_PETE_FEN, STARTING_FEN};
use rudim::perft::perft_test;

#[test]
#[ignore]
fn test_perft_starting_position() {
    perft_test(0, 1, STARTING_FEN);
    perft_test(1, 20, STARTING_FEN);
    perft_test(2, 400, STARTING_FEN);
    perft_test(3, 8_902, STARTING_FEN);
    perft_test(4, 197_281, STARTING_FEN);
    perft_test(5, 4_865_609, STARTING_FEN);
    perft_test(6, 119_060_324, STARTING_FEN);
}

#[test]
#[ignore]
fn test_perft_kiwi_pete() {
    perft_test(1, 48, KIWI_PETE_FEN);
    perft_test(2, 2_039, KIWI_PETE_FEN);
    perft_test(3, 97_862, KIWI_PETE_FEN);
    perft_test(4, 4_085_603, KIWI_PETE_FEN);
    perft_test(5, 193_690_690, KIWI_PETE_FEN);
}

#[test]
#[ignore]
fn test_perft_endgame() {
    perft_test(1, 14, ENDGAME_FEN);
    perft_test(2, 191, ENDGAME_FEN);
    perft_test(3, 2_812, ENDGAME_FEN);
    perft_test(4, 43_238, ENDGAME_FEN);
    perft_test(5, 674_624, ENDGAME_FEN);
    perft_test(6, 11_030_083, ENDGAME_FEN);
    perft_test(7, 178_633_661, ENDGAME_FEN);
}
