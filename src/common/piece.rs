#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

impl Piece {
    pub const ALL_PIECES: usize = 6;
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            6 => Piece::None,
            _ => panic!("Invalid piece index: {}", value),
        }
    }
}

impl From<Piece> for usize {
    fn from(p: Piece) -> Self {
        p as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_from_usize() {
        assert_eq!(Piece::from(0), Piece::Pawn);
        assert_eq!(Piece::from(5), Piece::King);
        assert_eq!(Piece::from(6), Piece::None);
    }

    #[test]
    #[should_panic(expected = "Invalid piece index: 7")]
    fn test_piece_from_usize_panics_on_invalid() {
        let _ = Piece::from(7);
    }

    #[test]
    fn test_piece_into_usize() {
        assert_eq!(usize::from(Piece::Pawn), 0);
        assert_eq!(usize::from(Piece::King), 5);
        assert_eq!(usize::from(Piece::None), 6);
    }
}
