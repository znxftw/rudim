use crate::bitboard::Bitboard;
use crate::common::constants::SQUARES;
use crate::common::square::Square;

include!(concat!(env!("OUT_DIR"), "/lookups_gen.rs"));

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
pub fn knight_attacks() -> &'static [u64; SQUARES] {
    &KNIGHT_ATTACKS
}
#[inline(always)]
pub fn king_attacks() -> &'static [u64; SQUARES] {
    &KING_ATTACKS
}
#[inline(always)]
fn bishop_attacks() -> &'static [[u64; 512]; SQUARES] {
    &BISHOP_ATTACKS
}
#[inline(always)]
fn rook_attacks() -> &'static [[u64; 4096]; SQUARES] {
    &ROOK_ATTACKS
}

pub fn init() {}

#[inline]
pub fn get_bishop_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    let sq = square as usize;
    let bits = bishop_mask_bits()[sq];
    let mask = bishop_masks()[sq];
    let index = crate::bitboard::magics::get_magic_index(
        Bitboard(occupancy.0 & mask),
        crate::bitboard::magics::BISHOP_MAGICS[sq],
        bits,
    );
    Bitboard(bishop_attacks()[sq][index])
}

#[inline]
pub fn get_rook_attacks_from_table(square: Square, occupancy: Bitboard) -> Bitboard {
    let sq = square as usize;
    let bits = rook_mask_bits()[sq];
    let mask = rook_masks()[sq];
    let index = crate::bitboard::magics::get_magic_index(
        Bitboard(occupancy.0 & mask),
        crate::bitboard::magics::ROOK_MAGICS[sq],
        bits,
    );
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
    use crate::{
        bitboard::attacks::{
            get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks,
            get_queen_attacks, get_rook_attacks,
        },
        common::side::Side,
    };

    #[test]
    fn pawn_table_matches_computed_for_all_squares() {
        for (sq, item) in PAWN_ATTACKS[Side::White as usize]
            .iter()
            .enumerate()
            .take(SQUARES)
        {
            let square = Square::from(sq);
            assert_eq!(
                get_pawn_attacks(square, Side::White).0,
                *item,
                "White pawn mismatch at sq={sq}"
            );
        }
        for (sq, item) in PAWN_ATTACKS[Side::Black as usize]
            .iter()
            .enumerate()
            .take(SQUARES)
        {
            let square = Square::from(sq);
            assert_eq!(
                get_pawn_attacks(square, Side::Black).0,
                *item,
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
