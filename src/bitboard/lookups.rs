use std::sync::LazyLock;

use crate::bitboard::Bitboard;
use crate::bitboard::attacks::{
    get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks, get_rook_attacks,
};
use crate::bitboard::magics::{
    BISHOP_MAGICS, ROOK_MAGICS, get_bishop_mask, get_occupancy_mapping, get_rook_mask,
};
use crate::common::constants::{MAX_BISHOP_MASK, MAX_ROOK_MASK, SQUARES};
use crate::common::side::Side;
use crate::common::square::Square;

static BISHOP_MASK_BITS: LazyLock<[u32; SQUARES]> = LazyLock::new(|| {
    let mut bits = [0u32; SQUARES];
    for sq in 0..SQUARES {
        bits[sq] = get_bishop_mask(Square::from(sq)).0.count_ones();
    }
    bits
});

static ROOK_MASK_BITS: LazyLock<[u32; SQUARES]> = LazyLock::new(|| {
    let mut bits = [0u32; SQUARES];
    for sq in 0..SQUARES {
        bits[sq] = get_rook_mask(Square::from(sq)).0.count_ones();
    }
    bits
});

pub static PAWN_ATTACKS: LazyLock<[[u64; SQUARES]; 2]> = LazyLock::new(|| {
    let mut table = [[0u64; SQUARES]; 2];
    for sq in 0..SQUARES {
        table[Side::White as usize][sq] = get_pawn_attacks(Square::from(sq), Side::White).0;
        table[Side::Black as usize][sq] = get_pawn_attacks(Square::from(sq), Side::Black).0;
    }
    table
});

pub static KNIGHT_ATTACKS: LazyLock<[u64; SQUARES]> = LazyLock::new(|| {
    let mut table = [0u64; SQUARES];
    for sq in 0..SQUARES {
        table[sq] = get_knight_attacks(Square::from(sq)).0;
    }
    table
});

pub static KING_ATTACKS: LazyLock<[u64; SQUARES]> = LazyLock::new(|| {
    let mut table = [0u64; SQUARES];
    for sq in 0..SQUARES {
        table[sq] = get_king_attacks(Square::from(sq)).0;
    }
    table
});

pub static BISHOP_ATTACKS: LazyLock<Vec<[u64; MAX_BISHOP_MASK]>> = LazyLock::new(|| {
    let mut table: Vec<[u64; MAX_BISHOP_MASK]> = vec![[0u64; MAX_BISHOP_MASK]; SQUARES];
    let bishop_mask_bits = &*BISHOP_MASK_BITS;

    for sq in 0..SQUARES {
        let mask = get_bishop_mask(Square::from(sq));
        let bits = bishop_mask_bits[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = occupancy
                .0
                .wrapping_mul(BISHOP_MAGICS[sq])
                .wrapping_shr(64 - bits) as usize;
            table[sq][magic_index] = get_bishop_attacks(Square::from(sq), occupancy).0;
        }
    }
    table
});

pub static ROOK_ATTACKS: LazyLock<Vec<[u64; MAX_ROOK_MASK]>> = LazyLock::new(|| {
    let mut table: Vec<[u64; MAX_ROOK_MASK]> = vec![[0u64; MAX_ROOK_MASK]; SQUARES];
    let rook_mask_bits = &*ROOK_MASK_BITS;

    for sq in 0..SQUARES {
        let mask = get_rook_mask(Square::from(sq));
        let bits = rook_mask_bits[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = occupancy
                .0
                .wrapping_mul(ROOK_MAGICS[sq])
                .wrapping_shr(64 - bits) as usize;
            table[sq][magic_index] = get_rook_attacks(Square::from(sq), occupancy).0;
        }
    }
    table
});

pub fn init() {
    let _ = &*BISHOP_MASK_BITS;
    let _ = &*ROOK_MASK_BITS;
    let _ = &*PAWN_ATTACKS;
    let _ = &*KNIGHT_ATTACKS;
    let _ = &*KING_ATTACKS;
    let _ = &*BISHOP_ATTACKS;
    let _ = &*ROOK_ATTACKS;
}

#[inline]
pub fn get_bishop_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    let sq = square as usize;
    let bits = BISHOP_MASK_BITS[sq];
    let mask = get_bishop_mask(square);
    let index = (occupancy.0 & mask.0)
        .wrapping_mul(BISHOP_MAGICS[sq])
        .wrapping_shr(64 - bits) as usize;
    Bitboard(BISHOP_ATTACKS[sq][index])
}

#[inline]
pub fn get_rook_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    let sq = square as usize;
    let bits = ROOK_MASK_BITS[sq];
    let mask = get_rook_mask(square);
    let index = (occupancy.0 & mask.0)
        .wrapping_mul(ROOK_MAGICS[sq])
        .wrapping_shr(64 - bits) as usize;
    Bitboard(ROOK_ATTACKS[sq][index])
}

#[inline]
pub fn get_queen_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    Bitboard(
        get_bishop_attacks_from_table(square, occupancy).0
            | get_rook_attacks_from_table(square, occupancy).0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::attacks::{get_bishop_attacks, get_queen_attacks, get_rook_attacks};

    #[test]
    fn pawn_table_matches_computed_for_all_squares() {
        for sq in 0..SQUARES {
            let square = Square::from(sq);
            assert_eq!(
                get_pawn_attacks(square, Side::White).0,
                PAWN_ATTACKS[Side::White as usize][sq],
                "White pawn mismatch at sq={sq}"
            );
            assert_eq!(
                get_pawn_attacks(square, Side::Black).0,
                PAWN_ATTACKS[Side::Black as usize][sq],
                "Black pawn mismatch at sq={sq}"
            );
        }
    }

    #[test]
    fn knight_table_matches_computed_for_all_squares() {
        for sq in 0..SQUARES {
            let square = Square::from(sq);
            assert_eq!(
                get_knight_attacks(square).0,
                KNIGHT_ATTACKS[sq],
                "Knight mismatch at sq={sq}"
            );
        }
    }

    #[test]
    fn king_table_matches_computed_for_all_squares() {
        for sq in 0..SQUARES {
            let square = Square::from(sq);
            assert_eq!(
                get_king_attacks(square).0,
                KING_ATTACKS[sq],
                "King mismatch at sq={sq}"
            );
        }
    }

    #[test]
    fn bishop_table_lookup_matches_computed_no_blockers() {
        let occupancy = Bitboard(0);
        for sq in 0..SQUARES {
            let square = Square::from(sq);
            let expected = get_bishop_attacks(square, occupancy).0;
            let actual = get_bishop_attacks_from_table(square, occupancy).0;
            assert_eq!(expected, actual, "Bishop (no blockers) mismatch at sq={sq}");
        }
    }

    #[test]
    fn bishop_table_lookup_matches_computed_with_blocker() {
        let mut occupancy = Bitboard(0);
        occupancy.set_bit(Square::D4 as usize);
        let square = Square::E5;
        let expected = get_bishop_attacks(square, occupancy).0;
        let actual = get_bishop_attacks_from_table(square, occupancy).0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn rook_table_lookup_matches_computed_no_blockers() {
        let occupancy = Bitboard(0);
        for sq in 0..SQUARES {
            let square = Square::from(sq);
            let expected = get_rook_attacks(square, occupancy).0;
            let actual = get_rook_attacks_from_table(square, occupancy).0;
            assert_eq!(expected, actual, "Rook (no blockers) mismatch at sq={sq}");
        }
    }

    #[test]
    fn rook_table_lookup_matches_computed_with_blockers() {
        let mut occupancy = Bitboard(0);
        occupancy.set_bit(Square::E3 as usize);
        occupancy.set_bit(Square::F5 as usize);
        let square = Square::E5;
        let expected = get_rook_attacks(square, occupancy).0;
        let actual = get_rook_attacks_from_table(square, occupancy).0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn queen_table_lookup_matches_computed_no_blockers() {
        let occupancy = Bitboard(0);
        for sq in 0..SQUARES {
            let square = Square::from(sq);
            let expected = get_queen_attacks(square, occupancy).0;
            let actual = get_queen_attacks_from_table(square, occupancy).0;
            assert_eq!(expected, actual, "Queen (no blockers) mismatch at sq={sq}");
        }
    }

    #[test]
    fn queen_table_lookup_matches_computed_with_blockers() {
        let mut occupancy = Bitboard(0);
        occupancy.set_bit(Square::D4 as usize);
        occupancy.set_bit(Square::E3 as usize);
        occupancy.set_bit(Square::F5 as usize);
        let square = Square::E5;
        let expected = get_queen_attacks(square, occupancy).0;
        let actual = get_queen_attacks_from_table(square, occupancy).0;
        assert_eq!(expected, actual);
    }
}
