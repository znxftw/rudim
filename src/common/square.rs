use std::fmt;

// TODO: Relook at optimizations here if reqd.
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
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

    pub fn mirrored(self) -> Self {
        if self == Square::NoSquare {
            return Square::NoSquare;
        }
        Square::from((self as usize) ^ 0b111000)
    }
}

impl From<usize> for Square {
    fn from(value: usize) -> Self {
        match value {
            0 => Square::A8,
            1 => Square::B8,
            2 => Square::C8,
            3 => Square::D8,
            4 => Square::E8,
            5 => Square::F8,
            6 => Square::G8,
            7 => Square::H8,
            8 => Square::A7,
            9 => Square::B7,
            10 => Square::C7,
            11 => Square::D7,
            12 => Square::E7,
            13 => Square::F7,
            14 => Square::G7,
            15 => Square::H7,
            16 => Square::A6,
            17 => Square::B6,
            18 => Square::C6,
            19 => Square::D6,
            20 => Square::E6,
            21 => Square::F6,
            22 => Square::G6,
            23 => Square::H6,
            24 => Square::A5,
            25 => Square::B5,
            26 => Square::C5,
            27 => Square::D5,
            28 => Square::E5,
            29 => Square::F5,
            30 => Square::G5,
            31 => Square::H5,
            32 => Square::A4,
            33 => Square::B4,
            34 => Square::C4,
            35 => Square::D4,
            36 => Square::E4,
            37 => Square::F4,
            38 => Square::G4,
            39 => Square::H4,
            40 => Square::A3,
            41 => Square::B3,
            42 => Square::C3,
            43 => Square::D3,
            44 => Square::E3,
            45 => Square::F3,
            46 => Square::G3,
            47 => Square::H3,
            48 => Square::A2,
            49 => Square::B2,
            50 => Square::C2,
            51 => Square::D2,
            52 => Square::E2,
            53 => Square::F2,
            54 => Square::G2,
            55 => Square::H2,
            56 => Square::A1,
            57 => Square::B1,
            58 => Square::C1,
            59 => Square::D1,
            60 => Square::E1,
            61 => Square::F1,
            62 => Square::G1,
            63 => Square::H1,
            64 => Square::NoSquare,
            _ => panic!("Invalid square index: {}", value),
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
    fn test_square_mirrored() {
        assert_eq!(Square::A8.mirrored(), Square::A1);
        assert_eq!(Square::A1.mirrored(), Square::A8);
        assert_eq!(Square::E4.mirrored(), Square::E5);
        assert_eq!(Square::E5.mirrored(), Square::E4);
        assert_eq!(Square::NoSquare.mirrored(), Square::NoSquare);
    }

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
