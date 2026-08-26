use crate::common::move_type::MoveType;
use crate::common::piece::Piece;
use crate::common::square::Square;

/// Compact 16-bit Move Encoding:
/// Bits 0..5 (6 bits): Source Square (0..63)
/// Bits 6..11 (6 bits): Target Square (0..63)
/// Bits 12..15 (4 bits): MoveType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Move(pub u16);

impl Move {
    pub const NO_MOVE: Move = Move(0);

    #[inline(always)]
    pub fn new(source: Square, target: Square, move_type: MoveType) -> Self {
        if source == Square::NoSquare || target == Square::NoSquare {
            return Move::NO_MOVE;
        }
        let src_idx = (source as u16) & 0x3F;
        let tgt_idx = (target as u16) & 0x3F;
        let mt_idx = (move_type as u16) & 0x0F;
        Move(src_idx | (tgt_idx << 6) | (mt_idx << 12))
    }

    #[inline(always)]
    pub fn source(self) -> Square {
        if self.0 == 0 {
            Square::NoSquare
        } else {
            Square::from((self.0 & 0x3F) as usize)
        }
    }

    #[inline(always)]
    pub fn target(self) -> Square {
        if self.0 == 0 {
            Square::NoSquare
        } else {
            Square::from(((self.0 >> 6) & 0x3F) as usize)
        }
    }

    #[inline(always)]
    pub fn move_type(self) -> MoveType {
        if self.0 == 0 {
            MoveType::Quiet
        } else {
            MoveType::from(((self.0 >> 12) & 0x0F) as u8)
        }
    }

    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.move_type().is_capture()
    }

    #[inline(always)]
    pub fn promotion_char(&self) -> Option<char> {
        self.move_type().promotion_char()
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        self.move_type().promotion_piece() != Piece::None
    }

    #[inline(always)]
    pub fn is_castle(&self) -> bool {
        self.move_type() == MoveType::Castle
    }

    pub fn parse_long_algebraic(move_string: &str) -> Option<Self> {
        if move_string.len() < 4 || move_string.len() > 5 {
            return None;
        }

        let parse_square = |s: &str| -> Option<Square> {
            let mut chars = s.chars();
            let f = chars.next()?.to_ascii_lowercase();
            let r = chars.next()?;

            if !('a'..='h').contains(&f) || !('1'..='8').contains(&r) {
                return None;
            }

            let file = (f as u8) - b'a';
            let rank = (r as u8) - b'1';

            let sq_idx = (7 - rank) * 8 + file;
            Some(Square::from(sq_idx as usize))
        };

        let source = parse_square(&move_string[0..2])?;
        let target = parse_square(&move_string[2..4])?;

        let move_type = if move_string.len() == 5 {
            match move_string.chars().nth(4)?.to_ascii_lowercase() {
                'q' => MoveType::QueenPromotion,
                'r' => MoveType::RookPromotion,
                'b' => MoveType::BishopPromotion,
                'n' => MoveType::KnightPromotion,
                _ => return None,
            }
        } else {
            MoveType::Quiet
        };

        Some(Self::new(source, target, move_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_size() {
        assert_eq!(std::mem::size_of::<Move>(), 2);
    }

    #[test]
    fn test_move_equality() {
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let m2 = Move::new(Square::E2, Square::E4, MoveType::Quiet);

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_parse_long_algebraic() {
        let m = Move::parse_long_algebraic("e2e4").unwrap();
        assert_eq!(m.source(), Square::E2);
        assert_eq!(m.target(), Square::E4);
        assert_eq!(m.move_type(), MoveType::Quiet);

        let m_prom = Move::parse_long_algebraic("e7e8q").unwrap();
        assert_eq!(m_prom.source(), Square::E7);
        assert_eq!(m_prom.target(), Square::E8);
        assert_eq!(m_prom.move_type(), MoveType::QueenPromotion);
    }

    #[test]
    fn test_properties() {
        let m_capture = Move::new(Square::E4, Square::D5, MoveType::Capture);
        assert!(m_capture.is_capture());
        assert!(!m_capture.is_promotion());
        assert!(!m_capture.is_castle());
        assert_eq!(m_capture.promotion_char(), None);

        let m_castle = Move::new(Square::E1, Square::G1, MoveType::Castle);
        assert!(!m_castle.is_capture());
        assert!(!m_castle.is_promotion());
        assert!(m_castle.is_castle());
        assert_eq!(m_castle.promotion_char(), None);

        let m_prom_cap = Move::new(Square::E7, Square::D8, MoveType::KnightPromotionCapture);
        assert!(m_prom_cap.is_capture());
        assert!(m_prom_cap.is_promotion());
        assert!(!m_prom_cap.is_castle());
        assert_eq!(m_prom_cap.promotion_char(), Some('n'));
    }

    #[test]
    fn test_equal_moves_should_be_equal() {
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let m2 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_moves_with_different_sources_should_not_be_equal() {
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let m2 = Move::new(Square::D2, Square::E4, MoveType::Quiet);
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_moves_with_different_targets_should_not_be_equal() {
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let m2 = Move::new(Square::E2, Square::E3, MoveType::Quiet);
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_moves_with_different_types_should_not_be_equal() {
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let m2 = Move::new(Square::E2, Square::E4, MoveType::Capture);
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_no_move_should_equal_itself() {
        let m1 = Move::NO_MOVE;
        let m2 = Move::NO_MOVE;
        assert_eq!(m1, m2);
        assert_eq!(m1.source(), Square::NoSquare);
        assert_eq!(m1.target(), Square::NoSquare);
    }
}
