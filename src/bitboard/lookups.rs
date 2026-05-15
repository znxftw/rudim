use crate::bitboard::Bitboard;
use crate::bitboard::attacks::{FILE_A, FILE_AB, FILE_GH, FILE_H};
use crate::bitboard::lookup_utils::{
    magic_index, mask_bits_for_masks, occupancy_mapping, ray_attacks, ray_mask_without_edges,
};
use crate::bitboard::magics::{BISHOP_MAGICS, ROOK_MAGICS};
use crate::common::constants::{MAX_BISHOP_MASK, MAX_ROOK_MASK, SQUARES};

use crate::common::square::Square;

static BISHOP_MASK_BITS: [u32; SQUARES] = mask_bits_for_masks(generate_bishop_masks());

static ROOK_MASK_BITS: [u32; SQUARES] = mask_bits_for_masks(generate_rook_masks());

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

static ROOK_MASKS: [u64; SQUARES] = generate_rook_masks();

const fn generate_rook_masks() -> [u64; SQUARES] {
    let mut masks = [0u64; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        masks[sq] = rook_mask_for_square(sq);
        sq += 1;
    }

    masks
}

const fn rook_mask_for_square(square: usize) -> u64 {
    let rook_rank = (square >> 3) as i32;
    let rook_file = (square & 7) as i32;

    ray_mask_without_edges(rook_rank, rook_file, 1, 0)
        | ray_mask_without_edges(rook_rank, rook_file, -1, 0)
        | ray_mask_without_edges(rook_rank, rook_file, 0, 1)
        | ray_mask_without_edges(rook_rank, rook_file, 0, -1)
}

static PAWN_ATTACKS: [[u64; SQUARES]; 2] = generate_pawn_attacks();

const WHITE: usize = 0;
const BLACK: usize = 1;

const fn generate_pawn_attacks() -> [[u64; SQUARES]; 2] {
    let mut table = [[0u64; SQUARES]; 2];
    let mut sq = 0;

    while sq < SQUARES {
        table[WHITE][sq] = pawn_attacks_for_square(sq, WHITE);
        table[BLACK][sq] = pawn_attacks_for_square(sq, BLACK);
        sq += 1;
    }

    table
}

const fn pawn_attacks_for_square(square: usize, side: usize) -> u64 {
    let pawn_board = 1u64 << square;
    let mut attacks = 0u64;

    if side == WHITE {
        attacks |= (pawn_board >> 9) & !FILE_H;
        attacks |= (pawn_board >> 7) & !FILE_A;
    } else {
        attacks |= (pawn_board << 7) & !FILE_H;
        attacks |= (pawn_board << 9) & !FILE_A;
    }

    attacks
}

static KNIGHT_ATTACKS: [u64; SQUARES] = generate_knight_attacks();

const fn generate_knight_attacks() -> [u64; SQUARES] {
    let mut table = [0u64; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        table[sq] = knight_attacks_for_square(sq);
        sq += 1;
    }

    table
}

const fn knight_attacks_for_square(square: usize) -> u64 {
    let knight_board = 1u64 << square;
    let mut attacks = 0u64;

    attacks |= (knight_board << 17) & !FILE_A;
    attacks |= (knight_board << 10) & !FILE_AB;
    attacks |= (knight_board >> 6) & !FILE_AB;
    attacks |= (knight_board >> 15) & !FILE_A;
    attacks |= (knight_board << 15) & !FILE_H;
    attacks |= (knight_board << 6) & !FILE_GH;
    attacks |= (knight_board >> 10) & !FILE_GH;
    attacks |= (knight_board >> 17) & !FILE_H;

    attacks
}

static KING_ATTACKS: [u64; SQUARES] = generate_king_attacks();

const fn generate_king_attacks() -> [u64; SQUARES] {
    let mut table = [0u64; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        table[sq] = king_attacks_for_square(sq);
        sq += 1;
    }

    table
}

const fn king_attacks_for_square(square: usize) -> u64 {
    let king_board = 1u64 << square;
    let mut attacks = 0u64;

    attacks |= (king_board << 1) & !FILE_A;
    attacks |= (king_board >> 7) & !FILE_A;
    attacks |= (king_board << 9) & !FILE_A;
    attacks |= (king_board >> 1) & !FILE_H;
    attacks |= (king_board << 7) & !FILE_H;
    attacks |= (king_board >> 9) & !FILE_H;
    attacks |= king_board << 8;
    attacks |= king_board >> 8;

    attacks
}

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

// TODO: recheck
#[allow(long_running_const_eval)]
static ROOK_ATTACKS: [[u64; MAX_ROOK_MASK]; SQUARES] = generate_rook_attacks();

const fn generate_rook_attacks() -> [[u64; MAX_ROOK_MASK]; SQUARES] {
    let mut table = [[0u64; MAX_ROOK_MASK]; SQUARES];
    let mut sq = 0;

    while sq < SQUARES {
        let mask = ROOK_MASKS[sq];
        let bits = ROOK_MASK_BITS[sq];
        let index_count = 1usize << bits;
        let mut index = 0;

        while index < index_count {
            let occupancy = occupancy_mapping(index, bits, mask);
            let table_index = magic_index(occupancy, ROOK_MAGICS[sq], bits);
            table[sq][table_index] = rook_attacks_for_occupancy(sq, occupancy);
            index += 1;
        }

        sq += 1;
    }

    table
}

const fn rook_attacks_for_occupancy(square: usize, occupancy: u64) -> u64 {
    let rook_rank = (square >> 3) as i32;
    let rook_file = (square & 7) as i32;

    ray_attacks(rook_rank, rook_file, 1, 0, occupancy)
        | ray_attacks(rook_rank, rook_file, -1, 0, occupancy)
        | ray_attacks(rook_rank, rook_file, 0, 1, occupancy)
        | ray_attacks(rook_rank, rook_file, 0, -1, occupancy)
}

#[inline(always)]
fn bishop_mask_bits() -> &'static [u32; SQUARES] {
    &BISHOP_MASK_BITS
}
#[inline(always)]
fn rook_mask_bits() -> &'static [u32; SQUARES] {
    &ROOK_MASK_BITS
}
#[inline(always)]
fn bishop_masks() -> &'static [u64; SQUARES] {
    &BISHOP_MASKS
}
#[inline(always)]
fn rook_masks() -> &'static [u64; SQUARES] {
    &ROOK_MASKS
}
#[inline(always)]
pub fn pawn_attacks() -> &'static [[u64; SQUARES]; 2] {
    &PAWN_ATTACKS
}
#[inline(always)]
pub fn knight_attacks() -> &'static [u64; SQUARES] {
    &KNIGHT_ATTACKS
}
#[inline(always)]
pub fn king_attacks() -> &'static [u64; SQUARES] {
    &KING_ATTACKS
}
#[inline(always)]
fn bishop_attacks() -> &'static [[u64; MAX_BISHOP_MASK]; SQUARES] {
    &BISHOP_ATTACKS
}
#[inline(always)]
fn rook_attacks() -> &'static [[u64; MAX_ROOK_MASK]; SQUARES] {
    &ROOK_ATTACKS
}

pub fn init() {
    let _ = &BISHOP_MASK_BITS;
    let _ = &ROOK_MASK_BITS;
    let _ = &BISHOP_MASKS;
    let _ = &ROOK_MASKS;
    let _ = &PAWN_ATTACKS;
    let _ = &KNIGHT_ATTACKS;
    let _ = &KING_ATTACKS;
    let _ = &BISHOP_ATTACKS;
    let _ = &ROOK_ATTACKS;
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
    use crate::bitboard::attacks::{
        get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks,
        get_queen_attacks, get_rook_attacks,
    };
    use crate::common::side::Side;

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
