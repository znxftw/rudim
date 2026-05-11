use std::sync::{Once, OnceLock};

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

static BISHOP_MASK_BITS: OnceLock<[u32; SQUARES]> = OnceLock::new();
static ROOK_MASK_BITS: OnceLock<[u32; SQUARES]> = OnceLock::new();
static BISHOP_MASKS: OnceLock<[u64; SQUARES]> = OnceLock::new();
static ROOK_MASKS: OnceLock<[u64; SQUARES]> = OnceLock::new();
static PAWN_ATTACKS: OnceLock<[[u64; SQUARES]; 2]> = OnceLock::new();
static KNIGHT_ATTACKS: OnceLock<[u64; SQUARES]> = OnceLock::new();
static KING_ATTACKS: OnceLock<[u64; SQUARES]> = OnceLock::new();
static BISHOP_ATTACKS: OnceLock<Vec<[u64; MAX_BISHOP_MASK]>> = OnceLock::new();
static ROOK_ATTACKS: OnceLock<Vec<[u64; MAX_ROOK_MASK]>> = OnceLock::new();

macro_rules! get_table {
    ($lock:expr) => {
        {
            #[cfg(debug_assertions)]
            {
                if $lock.get().is_none() {
                    crate::init();
                }
                $lock.get().unwrap()
            }
            #[cfg(not(debug_assertions))]
            {
                unsafe { $lock.get().unwrap_unchecked() }
            }
        }
    };
}

#[inline(always)] fn bishop_mask_bits() -> &'static [u32; SQUARES] { get_table!(BISHOP_MASK_BITS) }
#[inline(always)] fn rook_mask_bits() -> &'static [u32; SQUARES] { get_table!(ROOK_MASK_BITS) }
#[inline(always)] fn bishop_masks() -> &'static [u64; SQUARES] { get_table!(BISHOP_MASKS) }
#[inline(always)] fn rook_masks() -> &'static [u64; SQUARES] { get_table!(ROOK_MASKS) }
#[inline(always)] pub fn pawn_attacks() -> &'static [[u64; SQUARES]; 2] { get_table!(PAWN_ATTACKS) }
#[inline(always)] pub fn knight_attacks() -> &'static [u64; SQUARES] { get_table!(KNIGHT_ATTACKS) }
#[inline(always)] pub fn king_attacks() -> &'static [u64; SQUARES] { get_table!(KING_ATTACKS) }
#[inline(always)] fn bishop_attacks() -> &'static Vec<[u64; MAX_BISHOP_MASK]> { get_table!(BISHOP_ATTACKS) }
#[inline(always)] fn rook_attacks() -> &'static Vec<[u64; MAX_ROOK_MASK]> { get_table!(ROOK_ATTACKS) }

static INIT_ONCE: Once = Once::new();

pub fn init() {
    INIT_ONCE.call_once(|| {
        let mut bishop_mask_bits_table = [0u32; SQUARES];
    for (sq, entry) in bishop_mask_bits_table.iter_mut().enumerate() {
        *entry = get_bishop_mask(Square::from(sq)).0.count_ones();
    }
    BISHOP_MASK_BITS.set(bishop_mask_bits_table).unwrap();

    let mut rook_mask_bits_table = [0u32; SQUARES];
    for (sq, entry) in rook_mask_bits_table.iter_mut().enumerate() {
        *entry = get_rook_mask(Square::from(sq)).0.count_ones();
    }
    ROOK_MASK_BITS.set(rook_mask_bits_table).unwrap();

    let mut bishop_masks_table = [0u64; SQUARES];
    for (sq, entry) in bishop_masks_table.iter_mut().enumerate() {
        *entry = get_bishop_mask(Square::from(sq)).0;
    }
    BISHOP_MASKS.set(bishop_masks_table).unwrap();

    let mut rook_masks_table = [0u64; SQUARES];
    for (sq, entry) in rook_masks_table.iter_mut().enumerate() {
        *entry = get_rook_mask(Square::from(sq)).0;
    }
    ROOK_MASKS.set(rook_masks_table).unwrap();

    let mut pawn_attacks_table = [[0u64; SQUARES]; 2];
    for (sq, entry) in pawn_attacks_table[Side::White as usize].iter_mut().enumerate() {
        *entry = get_pawn_attacks(Square::from(sq), Side::White).0;
    }
    for (sq, entry) in pawn_attacks_table[Side::Black as usize].iter_mut().enumerate() {
        *entry = get_pawn_attacks(Square::from(sq), Side::Black).0;
    }
    PAWN_ATTACKS.set(pawn_attacks_table).unwrap();

    let mut knight_attacks_table = [0u64; SQUARES];
    for (sq, entry) in knight_attacks_table.iter_mut().enumerate() {
        *entry = get_knight_attacks(Square::from(sq)).0;
    }
    KNIGHT_ATTACKS.set(knight_attacks_table).unwrap();

    let mut king_attacks_table = [0u64; SQUARES];
    for (sq, entry) in king_attacks_table.iter_mut().enumerate() {
        *entry = get_king_attacks(Square::from(sq)).0;
    }
    KING_ATTACKS.set(king_attacks_table).unwrap();

    let mut bishop_attacks_table: Vec<[u64; MAX_BISHOP_MASK]> = vec![[0u64; MAX_BISHOP_MASK]; SQUARES];
    for sq in 0..SQUARES {
        let mask = get_bishop_mask(Square::from(sq));
        let bits = BISHOP_MASK_BITS.get().unwrap()[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = occupancy
                .0
                .wrapping_mul(BISHOP_MAGICS[sq])
                .wrapping_shr(64 - bits) as usize;
            bishop_attacks_table[sq][magic_index] = get_bishop_attacks(Square::from(sq), occupancy).0;
        }
    }
    BISHOP_ATTACKS.set(bishop_attacks_table).unwrap();

    let mut rook_attacks_table: Vec<[u64; MAX_ROOK_MASK]> = vec![[0u64; MAX_ROOK_MASK]; SQUARES];
    for sq in 0..SQUARES {
        let mask = get_rook_mask(Square::from(sq));
        let bits = ROOK_MASK_BITS.get().unwrap()[sq];
        let index_count = 1usize << bits;

        for index in 0..index_count {
            let occupancy = get_occupancy_mapping(index, bits as i32, mask);
            let magic_index = occupancy
                .0
                .wrapping_mul(ROOK_MAGICS[sq])
                .wrapping_shr(64 - bits) as usize;
            rook_attacks_table[sq][magic_index] = get_rook_attacks(Square::from(sq), occupancy).0;
        }
    }
        ROOK_ATTACKS.set(rook_attacks_table).unwrap();
    });
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
        init();
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
        init();
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
        init();
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
        init();
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
        init();
        let mut occupancy = Bitboard(0);
        occupancy.set_bit(Square::D4 as usize);
        let square = Square::E5;
        let expected = get_bishop_attacks(square, occupancy).0;
        let actual = get_bishop_attacks_from_table(square, occupancy).0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn rook_table_lookup_matches_computed_no_blockers() {
        init();
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
        init();
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
        init();
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
        init();
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
