use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Castle: u8 {
        const NONE = 0;
        const WHITE_SHORT = 1;
        const WHITE_LONG = 2;
        const BLACK_SHORT = 4;
        const BLACK_LONG = 8;
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
