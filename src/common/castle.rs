use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Castle(pub u8);

impl Castle {
    pub const NONE: Self = Self(0);
    pub const WHITE_SHORT: Self = Self(1);
    pub const WHITE_LONG: Self = Self(2);
    pub const BLACK_SHORT: Self = Self(4);
    pub const BLACK_LONG: Self = Self(8);

    #[inline]
    pub const fn bits(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn from_bits_retain(bits: u8) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl BitOr for Castle {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Castle {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Castle {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Castle {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_castle_bitflags() {
        let mut castle = Castle::NONE;
        assert_eq!(castle.bits(), 0);

        castle |= Castle::WHITE_SHORT;
        assert!(castle.contains(Castle::WHITE_SHORT));
        assert!(!castle.contains(Castle::WHITE_LONG));

        castle |= Castle::BLACK_LONG;
        assert!(castle.contains(Castle::WHITE_SHORT));
        assert!(castle.contains(Castle::BLACK_LONG));
        assert_eq!(castle.bits(), 1 | 8);

        castle.remove(Castle::WHITE_SHORT);
        assert!(!castle.contains(Castle::WHITE_SHORT));
        assert!(castle.contains(Castle::BLACK_LONG));
    }
}
