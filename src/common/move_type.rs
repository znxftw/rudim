use crate::common::piece::Piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MoveType {
    Quiet = 0,
    Capture = 1,
    EnPassant = 2,
    DoublePush = 3,
    KnightPromotion = 4,
    BishopPromotion = 5,
    RookPromotion = 6,
    QueenPromotion = 7,
    KnightPromotionCapture = 12,
    BishopPromotionCapture = 13,
    RookPromotionCapture = 14,
    QueenPromotionCapture = 15,
    Castle = 16,
}

impl MoveType {
    pub const fn value(self) -> u8 {
        self as u8
    }

    pub const fn promotion_piece(self) -> Piece {
        match self {
            Self::KnightPromotion | Self::KnightPromotionCapture => Piece::Knight,
            Self::BishopPromotion | Self::BishopPromotionCapture => Piece::Bishop,
            Self::RookPromotion | Self::RookPromotionCapture => Piece::Rook,
            Self::QueenPromotion | Self::QueenPromotionCapture => Piece::Queen,
            _ => Piece::None,
        }
    }

    pub const fn promotion_char(self) -> Option<char> {
        match self {
            Self::KnightPromotion | Self::KnightPromotionCapture => Some('n'),
            Self::BishopPromotion | Self::BishopPromotionCapture => Some('b'),
            Self::RookPromotion | Self::RookPromotionCapture => Some('r'),
            Self::QueenPromotion | Self::QueenPromotionCapture => Some('q'),
            _ => None,
        }
    }

    pub const fn is_capture(self) -> bool {
        matches!(
            self,
            Self::Capture
                | Self::EnPassant
                | Self::KnightPromotionCapture
                | Self::BishopPromotionCapture
                | Self::RookPromotionCapture
                | Self::QueenPromotionCapture
        )
    }

    pub const fn is_en_passant(self) -> bool {
        matches!(self, Self::EnPassant)
    }

    pub const fn is_double_push(self) -> bool {
        matches!(self, Self::DoublePush)
    }
}

impl From<u8> for MoveType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Quiet,
            1 => Self::Capture,
            2 => Self::EnPassant,
            3 => Self::DoublePush,
            4 => Self::KnightPromotion,
            5 => Self::BishopPromotion,
            6 => Self::RookPromotion,
            7 => Self::QueenPromotion,
            12 => Self::KnightPromotionCapture,
            13 => Self::BishopPromotionCapture,
            14 => Self::RookPromotionCapture,
            15 => Self::QueenPromotionCapture,
            16 => Self::Castle,
            _ => panic!("Invalid move type value: {}", value),
        }
    }
}

impl From<MoveType> for u8 {
    fn from(m: MoveType) -> Self {
        m as u8
    }
}

impl From<usize> for MoveType {
    fn from(value: usize) -> Self {
        Self::from(value as u8)
    }
}

impl From<MoveType> for usize {
    fn from(m: MoveType) -> Self {
        m as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_type_properties() {
        assert_eq!(MoveType::Quiet.value(), 0);
        assert_eq!(MoveType::Quiet.promotion_piece(), Piece::None);
        assert_eq!(MoveType::Quiet.promotion_char(), None);
        assert!(!MoveType::Quiet.is_capture());

        assert_eq!(MoveType::Capture.value(), 1);
        assert_eq!(MoveType::Capture.promotion_piece(), Piece::None);
        assert_eq!(MoveType::Capture.promotion_char(), None);
        assert!(MoveType::Capture.is_capture());

        assert_eq!(MoveType::EnPassant.value(), 2);
        assert_eq!(MoveType::EnPassant.promotion_piece(), Piece::None);
        assert_eq!(MoveType::EnPassant.promotion_char(), None);
        assert!(MoveType::EnPassant.is_capture());

        assert_eq!(MoveType::DoublePush.value(), 3);
        assert_eq!(MoveType::DoublePush.promotion_piece(), Piece::None);
        assert_eq!(MoveType::DoublePush.promotion_char(), None);
        assert!(!MoveType::DoublePush.is_capture());

        assert_eq!(MoveType::KnightPromotion.value(), 4);
        assert_eq!(MoveType::KnightPromotion.promotion_piece(), Piece::Knight);
        assert_eq!(MoveType::KnightPromotion.promotion_char(), Some('n'));
        assert!(!MoveType::KnightPromotion.is_capture());

        assert_eq!(MoveType::BishopPromotion.value(), 5);
        assert_eq!(MoveType::BishopPromotion.promotion_piece(), Piece::Bishop);
        assert_eq!(MoveType::BishopPromotion.promotion_char(), Some('b'));
        assert!(!MoveType::BishopPromotion.is_capture());

        assert_eq!(MoveType::RookPromotion.value(), 6);
        assert_eq!(MoveType::RookPromotion.promotion_piece(), Piece::Rook);
        assert_eq!(MoveType::RookPromotion.promotion_char(), Some('r'));
        assert!(!MoveType::RookPromotion.is_capture());

        assert_eq!(MoveType::QueenPromotion.value(), 7);
        assert_eq!(MoveType::QueenPromotion.promotion_piece(), Piece::Queen);
        assert_eq!(MoveType::QueenPromotion.promotion_char(), Some('q'));
        assert!(!MoveType::QueenPromotion.is_capture());

        assert_eq!(MoveType::KnightPromotionCapture.value(), 12);
        assert_eq!(
            MoveType::KnightPromotionCapture.promotion_piece(),
            Piece::Knight
        );
        assert_eq!(MoveType::KnightPromotionCapture.promotion_char(), Some('n'));
        assert!(MoveType::KnightPromotionCapture.is_capture());

        assert_eq!(MoveType::BishopPromotionCapture.value(), 13);
        assert_eq!(
            MoveType::BishopPromotionCapture.promotion_piece(),
            Piece::Bishop
        );
        assert_eq!(MoveType::BishopPromotionCapture.promotion_char(), Some('b'));
        assert!(MoveType::BishopPromotionCapture.is_capture());

        assert_eq!(MoveType::RookPromotionCapture.value(), 14);
        assert_eq!(
            MoveType::RookPromotionCapture.promotion_piece(),
            Piece::Rook
        );
        assert_eq!(MoveType::RookPromotionCapture.promotion_char(), Some('r'));
        assert!(MoveType::RookPromotionCapture.is_capture());

        assert_eq!(MoveType::QueenPromotionCapture.value(), 15);
        assert_eq!(
            MoveType::QueenPromotionCapture.promotion_piece(),
            Piece::Queen
        );
        assert_eq!(MoveType::QueenPromotionCapture.promotion_char(), Some('q'));
        assert!(MoveType::QueenPromotionCapture.is_capture());

        assert_eq!(MoveType::Castle.value(), 16);
        assert_eq!(MoveType::Castle.promotion_piece(), Piece::None);
        assert_eq!(MoveType::Castle.promotion_char(), None);
        assert!(!MoveType::Castle.is_capture());
    }

    #[test]
    fn test_move_type_conversions() {
        assert_eq!(MoveType::from(0_u8), MoveType::Quiet);
        assert_eq!(MoveType::from(15_u8), MoveType::QueenPromotionCapture);
        assert_eq!(u8::from(MoveType::Castle), 16);
    }

    #[test]
    #[should_panic(expected = "Invalid move type value: 99")]
    fn test_invalid_move_type_from_u8() {
        let _ = MoveType::from(99_u8);
    }

    #[test]
    #[should_panic(expected = "Invalid move type value: 99")]
    fn test_invalid_move_type_from_usize() {
        let _ = MoveType::from(99_usize);
    }
}
