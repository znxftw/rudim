pub mod attacks;
pub mod lookups;
pub mod magics;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
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
}

impl Default for Bitboard {
    fn default() -> Self {
        Self(0)
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
}
