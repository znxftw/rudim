use crate::bitboard::Bitboard;
use crate::common::side::Side;
use crate::common::square::Square;

pub const FILE_A: u64 = 72340172838076673;
pub const FILE_B: u64 = 144680345676153346;
pub const FILE_G: u64 = 4629771061636907072;
pub const FILE_H: u64 = 9259542123273814144;
pub const FILE_AB: u64 = FILE_A | FILE_B;
pub const FILE_GH: u64 = FILE_G | FILE_H;

pub fn get_pawn_attacks(square: Square, side: Side) -> Bitboard {
    let mut result_board = 0u64;
    let pawn_board = 1u64 << (square as usize);

    if side == Side::White {
        result_board |= (pawn_board >> 9) & !FILE_H;
        result_board |= (pawn_board >> 7) & !FILE_A;
    } else {
        result_board |= (pawn_board << 7) & !FILE_H;
        result_board |= (pawn_board << 9) & !FILE_A;
    }

    Bitboard(result_board)
}

pub fn get_knight_attacks(square: Square) -> Bitboard {
    let mut result_board = 0u64;
    let knight_board = 1u64 << (square as usize);

    result_board |= (knight_board << 17) & !FILE_A;
    result_board |= (knight_board << 10) & !FILE_AB;
    result_board |= (knight_board >> 6) & !FILE_AB;
    result_board |= (knight_board >> 15) & !FILE_A;
    result_board |= (knight_board << 15) & !FILE_H;
    result_board |= (knight_board << 6) & !FILE_GH;
    result_board |= (knight_board >> 10) & !FILE_GH;
    result_board |= (knight_board >> 17) & !FILE_H;

    Bitboard(result_board)
}

pub fn get_king_attacks(square: Square) -> Bitboard {
    let mut result_board = 0u64;
    let king_board = 1u64 << (square as usize);

    result_board |= (king_board << 1) & !FILE_A;
    result_board |= (king_board >> 7) & !FILE_A;
    result_board |= (king_board << 9) & !FILE_A;

    result_board |= (king_board >> 1) & !FILE_H;
    result_board |= (king_board << 7) & !FILE_H;
    result_board |= (king_board >> 9) & !FILE_H;

    result_board |= king_board << 8;
    result_board |= king_board >> 8;

    Bitboard(result_board)
}

pub(crate) fn add_square_to_board_and_stop_at_occupied_square(
    result_board: &mut u64,
    rank: i32,
    file: i32,
    occupancy: Bitboard,
) -> bool {
    *result_board |= 1u64 << ((rank * 8) + file);
    (1u64 << ((rank * 8) + file) & occupancy.0) > 0
}

pub fn get_bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut result_board = 0u64;
    let sq = square as i32;
    let bishop_rank = sq >> 3;
    let bishop_file = sq & (8 - 1);

    for (rank, file) in ((bishop_rank + 1)..8).zip((bishop_file + 1)..8) {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rank, file, occupancy) {
            break;
        }
    }

    for (rank, file) in (0..bishop_rank).rev().zip((bishop_file + 1)..8) {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rank, file, occupancy) {
            break;
        }
    }

    for (rank, file) in (0..bishop_rank).rev().zip((0..bishop_file).rev()) {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rank, file, occupancy) {
            break;
        }
    }

    for (rank, file) in ((bishop_rank + 1)..8).zip((0..bishop_file).rev()) {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rank, file, occupancy) {
            break;
        }
    }

    Bitboard(result_board)
}

pub fn get_rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut result_board = 0u64;
    let sq = square as i32;
    let rook_rank = sq >> 3;
    let rook_file = sq & (8 - 1);

    for rank in (rook_rank + 1)..8 {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rank, rook_file, occupancy) {
            break;
        }
    }

    for rank in (0..rook_rank).rev() {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rank, rook_file, occupancy) {
            break;
        }
    }

    for file in (rook_file + 1)..8 {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rook_rank, file, occupancy) {
            break;
        }
    }

    for file in (0..rook_file).rev() {
        if add_square_to_board_and_stop_at_occupied_square(&mut result_board, rook_rank, file, occupancy) {
            break;
        }
    }

    Bitboard(result_board)
}

pub fn get_queen_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let rook_attacks = get_rook_attacks(square, occupancy);
    let bishop_attacks = get_bishop_attacks(square, occupancy);
    Bitboard(rook_attacks.0 | bishop_attacks.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_attacks_for_central_pawn() {
        let pawn_attacks_white = get_pawn_attacks(Square::E5, Side::White);
        let pawn_attacks_black = get_pawn_attacks(Square::E5, Side::Black);

        assert_eq!(1, pawn_attacks_white.get_bit(Square::F6 as usize));
        assert_eq!(1, pawn_attacks_white.get_bit(Square::D6 as usize));
        assert_eq!(1, pawn_attacks_black.get_bit(Square::F4 as usize));
        assert_eq!(1, pawn_attacks_black.get_bit(Square::D4 as usize));
        assert_eq!(2, pawn_attacks_black.0.count_ones());
        assert_eq!(2, pawn_attacks_white.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_white_corner_pawn() {
        let pawn_attacks_white_a1 = get_pawn_attacks(Square::A1, Side::White);
        let pawn_attacks_white_a8 = get_pawn_attacks(Square::A8, Side::White);

        assert_eq!(1, pawn_attacks_white_a1.get_bit(Square::B2 as usize));
        assert_eq!(1, pawn_attacks_white_a1.0.count_ones());

        assert_eq!(0, pawn_attacks_white_a8.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_black_corner_pawn() {
        let pawn_attacks_black_a1 = get_pawn_attacks(Square::A1, Side::Black);
        let pawn_attacks_black_a8 = get_pawn_attacks(Square::A8, Side::Black);

        assert_eq!(0, pawn_attacks_black_a1.0.count_ones());

        assert_eq!(1, pawn_attacks_black_a8.get_bit(Square::B7 as usize));
        assert_eq!(1, pawn_attacks_black_a8.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_central_knight() {
        let knight_attacks_e5 = get_knight_attacks(Square::E5);

        assert_eq!(1, knight_attacks_e5.get_bit(Square::F3 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::G4 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::G6 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::F7 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::D7 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::C6 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::C4 as usize));
        assert_eq!(1, knight_attacks_e5.get_bit(Square::D3 as usize));
        assert_eq!(8, knight_attacks_e5.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_corner_knight() {
        let knight_attacks_a1 = get_knight_attacks(Square::A1);

        assert_eq!(1, knight_attacks_a1.get_bit(Square::B3 as usize));
        assert_eq!(1, knight_attacks_a1.get_bit(Square::C2 as usize));
        assert_eq!(2, knight_attacks_a1.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_central_king() {
        let king_attacks_e5 = get_king_attacks(Square::E5);

        assert_eq!(1, king_attacks_e5.get_bit(Square::E4 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::E6 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::F4 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::F5 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::F6 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::D4 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::D5 as usize));
        assert_eq!(1, king_attacks_e5.get_bit(Square::D6 as usize));
        assert_eq!(8, king_attacks_e5.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_corner_king() {
        let king_attacks_a1 = get_king_attacks(Square::A1);

        assert_eq!(1, king_attacks_a1.get_bit(Square::A2 as usize));
        assert_eq!(1, king_attacks_a1.get_bit(Square::B1 as usize));
        assert_eq!(1, king_attacks_a1.get_bit(Square::B2 as usize));
        assert_eq!(3, king_attacks_a1.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_central_bishop_with_no_blockers() {
        let occupancy_board = Bitboard(0);
        let bishop_attacks_e5 = get_bishop_attacks(Square::E5, occupancy_board);

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::F4 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::G3 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::H2 as usize));

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::F6 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::G7 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::H8 as usize));

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::D4 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::C3 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::B2 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::A1 as usize));

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::D6 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::C7 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::B8 as usize));

        assert_eq!(13, bishop_attacks_e5.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_central_bishop_with_blockers() {
        let mut occupancy_board = Bitboard(0);
        occupancy_board.set_bit(Square::D4 as usize); // Should prune c3,b2,a1
        occupancy_board.set_bit(Square::A5 as usize); // Should not cause any problems because it is not in the diagonal
        occupancy_board.set_bit(Square::H2 as usize); // Should not change anything as it is an edge square
        let bishop_attacks_e5 = get_bishop_attacks(Square::E5, occupancy_board);

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::F4 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::G3 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::H2 as usize));

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::F6 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::G7 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::H8 as usize));

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::D4 as usize));

        assert_eq!(1, bishop_attacks_e5.get_bit(Square::D6 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::C7 as usize));
        assert_eq!(1, bishop_attacks_e5.get_bit(Square::B8 as usize));

        assert_eq!(10, bishop_attacks_e5.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_corner_bishop_with_no_blockers() {
        let occupancy_board = Bitboard(0);
        let bishop_attacks_a1 = get_bishop_attacks(Square::A1, occupancy_board);

        assert_eq!(1, bishop_attacks_a1.get_bit(Square::B2 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::C3 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::D4 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::E5 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::F6 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::G7 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::H8 as usize));
        assert_eq!(7, bishop_attacks_a1.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_corner_bishop_with_blockers() {
        let mut occupancy_board = Bitboard(0);
        occupancy_board.set_bit(Square::E5 as usize); // Should prune f6, g7, h8
        occupancy_board.set_bit(Square::E4 as usize); // Should not make a difference
        let bishop_attacks_a1 = get_bishop_attacks(Square::A1, occupancy_board);

        assert_eq!(1, bishop_attacks_a1.get_bit(Square::B2 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::C3 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::D4 as usize));
        assert_eq!(1, bishop_attacks_a1.get_bit(Square::E5 as usize));
        assert_eq!(4, bishop_attacks_a1.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_central_rook_with_no_blockers() {
        let occupancy_board = Bitboard(0);
        let rook_attacks_e5 = get_rook_attacks(Square::E5, occupancy_board);

        assert_eq!(1, rook_attacks_e5.get_bit(Square::E1 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E2 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E3 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E4 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E6 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E7 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E8 as usize));

        assert_eq!(1, rook_attacks_e5.get_bit(Square::A5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::B5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::C5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::D5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::F5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::G5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::H5 as usize));

        assert_eq!(14, rook_attacks_e5.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_central_rook_with_blockers() {
        let mut occupancy_board = Bitboard(0);
        occupancy_board.set_bit(Square::E3 as usize); // Should prune e2, e1
        occupancy_board.set_bit(Square::G7 as usize); // Should not make a difference
        occupancy_board.set_bit(Square::E8 as usize); // Should not make a difference
        occupancy_board.set_bit(Square::F5 as usize); // Should prune g5, h5
        let rook_attacks_e5 = get_rook_attacks(Square::E5, occupancy_board);

        assert_eq!(1, rook_attacks_e5.get_bit(Square::E3 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E4 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E6 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E7 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::E8 as usize));

        assert_eq!(1, rook_attacks_e5.get_bit(Square::A5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::B5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::C5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::D5 as usize));
        assert_eq!(1, rook_attacks_e5.get_bit(Square::F5 as usize));

        assert_eq!(10, rook_attacks_e5.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_corner_rook_with_no_blockers() {
        let occupancy_board = Bitboard(0);
        let rook_attacks_a1 = get_rook_attacks(Square::A1, occupancy_board);

        assert_eq!(1, rook_attacks_a1.get_bit(Square::A2 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A3 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A4 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A5 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A6 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A7 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A8 as usize));

        assert_eq!(1, rook_attacks_a1.get_bit(Square::B1 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::C1 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::D1 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::E1 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::F1 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::G1 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::H1 as usize));

        assert_eq!(14, rook_attacks_a1.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_corner_rook_with_blockers() {
        let mut occupancy_board = Bitboard(0);
        occupancy_board.set_bit(Square::A5 as usize); // Should prune a6, a7, a8
        occupancy_board.set_bit(Square::G7 as usize); // Should not make a difference
        occupancy_board.set_bit(Square::E8 as usize); // Should not make a difference
        occupancy_board.set_bit(Square::B1 as usize); // Should prune c1, d1, e1, f1, g1, h1
        let rook_attacks_a1 = get_rook_attacks(Square::A1, occupancy_board);

        assert_eq!(1, rook_attacks_a1.get_bit(Square::A2 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A3 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A4 as usize));
        assert_eq!(1, rook_attacks_a1.get_bit(Square::A5 as usize));

        assert_eq!(1, rook_attacks_a1.get_bit(Square::B1 as usize));

        assert_eq!(5, rook_attacks_a1.0.count_ones());
    }

    #[test]
    fn should_get_attacks_for_queen() {
        let occupancy_board = Bitboard(0);
        let expected_board = get_bishop_attacks(Square::E5, occupancy_board).0
            | get_rook_attacks(Square::E5, occupancy_board).0;

        let queen_attacks_e5 = get_queen_attacks(Square::E5, occupancy_board);

        assert_eq!(expected_board, queen_attacks_e5.0);
    }
}
