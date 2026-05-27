pub mod attacks;
pub mod lookups;
pub mod magics;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Self = Bitboard(0);

    #[inline]
    pub fn get_bit(&self, square: usize) -> u8 {
        ((self.0 >> square) & 1) as u8
    }

    #[inline]
    pub fn set_bit(&mut self, square: usize) {
        self.0 |= 1u64 << square;
    }

    #[inline]
    pub fn clear_bit(&mut self, square: usize) {
        self.0 &= !(1u64 << square);
    }

    #[inline]
    pub fn get_lsb(&self) -> u32 {
        self.0.trailing_zeros()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn clear_lsb(&mut self) {
        self.0 &= self.0 - 1;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: u64) -> Self {
        Bitboard(self.0 & rhs)
    }
}

impl BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<u64> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: u64) -> Self {
        Bitboard(self.0 | rhs)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u64> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: u64) -> Self {
        Bitboard(self.0 ^ rhs)
    }
}

impl BitXorAssign for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<u64> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}

impl Not for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self {
        Bitboard(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_specified_bits() {
        let mut board = Bitboard(0);

        board.set_bit(5);
        assert_eq!(32, board.0);

        board.set_bit(63);
        assert_eq!(9223372036854775840, board.0);
    }

    #[test]
    fn should_clear_specified_bits() {
        let mut board = Bitboard(9223372036854775840);

        board.clear_bit(63);
        assert_eq!(32, board.0);

        board.clear_bit(5);
        assert_eq!(0, board.0);
    }

    #[test]
    fn set_bit_should_be_idempotent() {
        let mut board = Bitboard(0);

        board.set_bit(63);
        assert_eq!(9223372036854775808, board.0);

        board.set_bit(63);
        assert_eq!(9223372036854775808, board.0);
    }

    #[test]
    fn clear_bit_should_be_idempotent() {
        let mut board = Bitboard(9223372036854775808);

        board.clear_bit(63);
        assert_eq!(0, board.0);

        board.clear_bit(63);
        assert_eq!(0, board.0);
    }

    #[test]
    fn should_get_given_bits() {
        let board = Bitboard(9223372036854775808);

        assert_eq!(0, board.get_bit(0));
        assert_eq!(0, board.get_bit(5));
        assert_eq!(1, board.get_bit(63));
    }

    #[test]
    fn should_get_lsb() {
        let board1 = Bitboard(1);
        assert_eq!(0, board1.get_lsb());

        let board2 = Bitboard(32);
        assert_eq!(5, board2.get_lsb());

        let board3 = Bitboard(9223372036854775808);
        assert_eq!(63, board3.get_lsb());

        let board4 = Bitboard(0);
        assert_eq!(64, board4.get_lsb());
    }

    #[test]
    fn bitwise_operators_should_work() {
        let b1 = Bitboard(0b1100);
        let b2 = Bitboard(0b1010);

        assert_eq!(b1 & b2, Bitboard(0b1000));
        assert_eq!(b1 | b2, Bitboard(0b1110));
        assert_eq!(b1 ^ b2, Bitboard(0b0110));
        assert_eq!(!Bitboard(0), Bitboard(u64::MAX));

        // Mixed with u64
        assert_eq!(b1 & 0b1010, Bitboard(0b1000));
        assert_eq!(b1 | 0b1010, Bitboard(0b1110));
        assert_eq!(b1 ^ 0b1010, Bitboard(0b0110));

        // Assign operators
        let mut temp = b1;
        temp &= b2;
        assert_eq!(temp, Bitboard(0b1000));

        let mut temp = b1;
        temp |= b2;
        assert_eq!(temp, Bitboard(0b1110));

        let mut temp = b1;
        temp ^= b2;
        assert_eq!(temp, Bitboard(0b0110));

        // Assign operators with u64
        let mut temp = b1;
        temp &= 0b1010;
        assert_eq!(temp, Bitboard(0b1000));

        let mut temp = b1;
        temp |= 0b1010;
        assert_eq!(temp, Bitboard(0b1110));

        let mut temp = b1;
        temp ^= 0b1010;
        assert_eq!(temp, Bitboard(0b0110));
    }

    #[test]
    fn empty_helpers_should_work() {
        assert!(Bitboard::EMPTY.is_empty());
        assert!(!Bitboard::EMPTY.is_not_empty());

        assert!(!Bitboard(42).is_empty());
        assert!(Bitboard(42).is_not_empty());
    }
}
