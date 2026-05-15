use std::sync::LazyLock;

use crate::bitboard::Bitboard;
use crate::bitboard::attacks::{
    get_king_attacks, get_knight_attacks, get_pawn_attacks, get_rook_attacks,
};
use crate::bitboard::lookup_utils::{
    magic_index, mask_bits_for_masks, occupancy_mapping, ray_attacks, ray_mask_without_edges,
};
use crate::bitboard::magics::{BISHOP_MAGICS, ROOK_MAGICS, get_occupancy_mapping, get_rook_mask};
use crate::common::constants::{MAX_BISHOP_MASK, MAX_ROOK_MASK, SQUARES};
use crate::common::side::Side;
use crate::common::square::Square;

static BISHOP_MASK_BITS: [u32; SQUARES] = mask_bits_for_masks(generate_bishop_masks());

static ROOK_MASK_BITS: LazyLock<[u32; SQUARES]> = LazyLock::new(|| {
    let mut bits = [0u32; SQUARES];
    for (sq, entry) in bits.iter_mut().enumerate() {
        *entry = get_rook_mask(Square::from(sq)).0.count_ones();
    }
    bits
});

static BISHOP_MASKS: [u64; SQUARES] = generate_bishop_masks();

const fn generate_bishop_masks() -> [u64; SQUARES] {
    let mut masks = [0u64; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        masks[sq] = bishop_mask_for_square(sq);
        sq += 1;
    }

    masks
}

const fn bishop_mask_for_square(square: usize) -> u64 {
    let bishop_rank = (square >> 3) as i32;
    let bishop_file = (square & 7) as i32;

    ray_mask_without_edges(bishop_rank, bishop_file, 1, 1)
        | ray_mask_without_edges(bishop_rank, bishop_file, -1, 1)
        | ray_mask_without_edges(bishop_rank, bishop_file, -1, -1)
        | ray_mask_without_edges(bishop_rank, bishop_file, 1, -1)
}

static ROOK_MASKS: LazyLock<[u64; SQUARES]> = LazyLock::new(|| {
    let mut masks = [0u64; SQUARES];
    for (sq, entry) in masks.iter_mut().enumerate() {
        *entry = get_rook_mask(Square::from(sq)).0;
    }
    masks
});

static PAWN_ATTACKS: LazyLock<[[u64; SQUARES]; 2]> = LazyLock::new(|| {
    let mut table = [[0u64; SQUARES]; 2];
    for (sq, entry) in table[Side::White as usize].iter_mut().enumerate() {
        *entry = get_pawn_attacks(Square::from(sq), Side::White).0;
    }
    for (sq, entry) in table[Side::Black as usize].iter_mut().enumerate() {
        *entry = get_pawn_attacks(Square::from(sq), Side::Black).0;
    }
    table
});

static KNIGHT_ATTACKS: LazyLock<[u64; SQUARES]> = LazyLock::new(|| {
    let mut table = [0u64; SQUARES];
    for (sq, entry) in table.iter_mut().enumerate() {
        *entry = get_knight_attacks(Square::from(sq)).0;
    }
    table
});

static KING_ATTACKS: LazyLock<[u64; SQUARES]> = LazyLock::new(|| {
    let mut table = [0u64; SQUARES];
    for (sq, entry) in table.iter_mut().enumerate() {
        *entry = get_king_attacks(Square::from(sq)).0;
    }
    table
});

static BISHOP_ATTACKS: [[u64; MAX_BISHOP_MASK]; SQUARES] = generate_bishop_attacks();

const fn generate_bishop_attacks() -> [[u64; MAX_BISHOP_MASK]; SQUARES] {
    let mut table = [[0u64; MAX_BISHOP_MASK]; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        let mask = BISHOP_MASKS[sq];
        let bits = BISHOP_MASK_BITS[sq];
        let index_count = 1usize << bits;
        let mut index = 0;

        while index < index_count {
            let occupancy = occupancy_mapping(index, bits, mask);
            let table_index = magic_index(occupancy, BISHOP_MAGICS[sq], bits);
            table[sq][table_index] = bishop_attacks_for_occupancy(sq, occupancy);
            index += 1;
        }

        sq += 1;
    }

    table
}

const fn bishop_attacks_for_occupancy(square: usize, occupancy: u64) -> u64 {
    let bishop_rank = (square >> 3) as i32;
    let bishop_file = (square & 7) as i32;

    ray_attacks(bishop_rank, bishop_file, 1, 1, occupancy)
        | ray_attacks(bishop_rank, bishop_file, -1, 1, occupancy)
        | ray_attacks(bishop_rank, bishop_file, -1, -1, occupancy)
        | ray_attacks(bishop_rank, bishop_file, 1, -1, occupancy)
}

static ROOK_ATTACKS: LazyLock<Vec<[u64; MAX_ROOK_MASK]>> = LazyLock::new(|| {
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

macro_rules! get_table {
    ($lock:expr) => {{
        #[cfg(debug_assertions)]
        {
            LazyLock::force(&$lock)
        }
        #[cfg(not(debug_assertions))]
        {
            // SAFETY: `rudim::init()` must be called at program startup, which forces this LazyLock to initialize.
            // get() will hence always return a value.
            unsafe { LazyLock::get(&$lock).unwrap_unchecked() }
        }
    }};
}

#[inline(always)]
fn bishop_mask_bits() -> &'static [u32; SQUARES] {
    &BISHOP_MASK_BITS
}
#[inline(always)]
fn rook_mask_bits() -> &'static [u32; SQUARES] {
    get_table!(ROOK_MASK_BITS)
}
#[inline(always)]
fn bishop_masks() -> &'static [u64; SQUARES] {
    &BISHOP_MASKS
}
#[inline(always)]
fn rook_masks() -> &'static [u64; SQUARES] {
    get_table!(ROOK_MASKS)
}
#[inline(always)]
pub fn pawn_attacks() -> &'static [[u64; SQUARES]; 2] {
    get_table!(PAWN_ATTACKS)
}
#[inline(always)]
pub fn knight_attacks() -> &'static [u64; SQUARES] {
    get_table!(KNIGHT_ATTACKS)
}
#[inline(always)]
pub fn king_attacks() -> &'static [u64; SQUARES] {
    get_table!(KING_ATTACKS)
}
#[inline(always)]
fn bishop_attacks() -> &'static [[u64; MAX_BISHOP_MASK]; SQUARES] {
    &BISHOP_ATTACKS
}
#[inline(always)]
fn rook_attacks() -> &'static Vec<[u64; MAX_ROOK_MASK]> {
    get_table!(ROOK_ATTACKS)
}

pub fn init() {
    let _ = &BISHOP_MASK_BITS;
    let _ = &*ROOK_MASK_BITS;
    let _ = &BISHOP_MASKS;
    let _ = &*ROOK_MASKS;
    let _ = &*PAWN_ATTACKS;
    let _ = &*KNIGHT_ATTACKS;
    let _ = &*KING_ATTACKS;
    let _ = &BISHOP_ATTACKS;
    let _ = &*ROOK_ATTACKS;
}

#[inline]
pub fn get_bishop_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    let sq = square as usize;
    let bits = bishop_mask_bits()[sq];
    let mask = bishop_masks()[sq];
    let index = (occupancy.0 & mask)
        .wrapping_mul(BISHOP_MAGICS[sq])
        .wrapping_shr(64 - bits) as usize;
    Bitboard(bishop_attacks()[sq][index])
}

#[inline]
pub fn get_rook_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    let sq = square as usize;
    let bits = rook_mask_bits()[sq];
    let mask = rook_masks()[sq];
    let index = (occupancy.0 & mask)
        .wrapping_mul(ROOK_MAGICS[sq])
        .wrapping_shr(64 - bits) as usize;
    Bitboard(rook_attacks()[sq][index])
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
                pawn_attacks()[Side::White as usize][sq],
                "White pawn mismatch at sq={sq}"
            );
            assert_eq!(
                get_pawn_attacks(square, Side::Black).0,
                pawn_attacks()[Side::Black as usize][sq],
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
                knight_attacks()[sq],
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
                king_attacks()[sq],
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
