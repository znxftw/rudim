use crate::common::move_type::MoveType;
use crate::common::piece::Piece;
use crate::common::square::Square;
use std::hash::{Hash, Hasher};

// TODO: optimize memory here, can save a lot of TT space
// ref for ideas https://github.com/codedeliveryservice/Reckless/blob/main/src/types/moves.rs
#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub source: Square,
    pub target: Square,
    pub move_type: MoveType,
    pub score: i32,
}

impl Move {
    pub const NO_MOVE: Move = Move {
        source: Square::NoSquare,
        target: Square::NoSquare,
        move_type: MoveType::Quiet,
        score: 0,
    };

    pub fn new(source: Square, target: Square, move_type: MoveType) -> Self {
        Self {
            source,
            target,
            move_type,
            score: 0,
        }
    }

    pub fn is_capture(&self) -> bool {
        self.move_type.is_capture()
    }

    pub fn promotion_char(&self) -> Option<char> {
        self.move_type.promotion_char()
    }

    pub fn is_promotion(&self) -> bool {
        self.move_type.promotion_piece() != Piece::None
    }

    pub fn is_castle(&self) -> bool {
        self.move_type == MoveType::Castle
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

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.target == other.target
            && self.move_type == other.move_type
    }
}

impl Eq for Move {}

impl Hash for Move {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.target.hash(state);
        self.move_type.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_equality() {
        let m1 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        let mut m2 = Move::new(Square::E2, Square::E4, MoveType::Quiet);
        m2.score = 100;

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_parse_long_algebraic() {
        let m = Move::parse_long_algebraic("e2e4").unwrap();
        assert_eq!(m.source, Square::E2);
        assert_eq!(m.target, Square::E4);
        assert_eq!(m.move_type, MoveType::Quiet);

        let m_prom = Move::parse_long_algebraic("e7e8q").unwrap();
        assert_eq!(m_prom.source, Square::E7);
        assert_eq!(m_prom.target, Square::E8);
        assert_eq!(m_prom.move_type, MoveType::QueenPromotion);
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
    }
}
