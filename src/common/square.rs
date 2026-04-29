use std::fmt;

// TODO: Relook at optimizations here if reqd.
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Square {
    A8 = 0, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    NoSquare = 64,
}

impl Square {
    pub const ALL_SQUARES: usize = 64;
}

#[rustfmt::skip]
const SQUARES: [Square; 65] = [
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::NoSquare,
];

impl From<usize> for Square {
    fn from(value: usize) -> Self {
        if value <= 64 {
            SQUARES[value]
        } else {
            panic!("Invalid square index: {}", value)
        }
    }
}

impl From<Square> for usize {
    fn from(sq: Square) -> Self {
        sq as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Square::NoSquare {
            return write!(f, "-");
        }

        let sq = *self as usize;
        let file = (sq % 8) as u8;
        let rank = 7 - (sq / 8) as u8;

        let file_char = (b'a' + file) as char;
        let rank_char = (b'1' + rank) as char;

        write!(f, "{}{}", file_char, rank_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_usize() {
        assert_eq!(Square::from(0), Square::A8);
        assert_eq!(Square::from(28), Square::E5);
        assert_eq!(Square::from(63), Square::H1);
        assert_eq!(Square::from(64), Square::NoSquare);
    }

    #[test]
    #[should_panic(expected = "Invalid square index: 65")]
    fn test_square_from_usize_panics_on_invalid() {
        let _ = Square::from(65);
    }

    #[test]
    fn test_square_into_usize() {
        assert_eq!(usize::from(Square::A8), 0);
        assert_eq!(usize::from(Square::E5), 28);
        assert_eq!(usize::from(Square::H1), 63);
        assert_eq!(usize::from(Square::NoSquare), 64);
    }

    #[test]
    fn test_square_display() {
        assert_eq!(Square::A8.to_string(), "a8");
        assert_eq!(Square::H1.to_string(), "h1");
        assert_eq!(Square::E4.to_string(), "e4");
        assert_eq!(Square::NoSquare.to_string(), "-");
    }
}
