#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Side {
    White = 0,
    Black,
    Both,
}

impl Side {
    pub fn other(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
            Side::Both => Side::Both,
        }
    }
}

const SIDES: [Side; 3] = [Side::White, Side::Black, Side::Both];

impl From<usize> for Side {
    fn from(value: usize) -> Self {
        if value <= 2 {
            SIDES[value]
        } else {
            panic!("Invalid side index: {}", value)
        }
    }
}

impl From<Side> for usize {
    fn from(s: Side) -> Self {
        s as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_other() {
        assert_eq!(Side::White.other(), Side::Black);
        assert_eq!(Side::Black.other(), Side::White);
        assert_eq!(Side::Both.other(), Side::Both);
    }

    #[test]
    fn test_side_from_usize() {
        assert_eq!(Side::from(0), Side::White);
        assert_eq!(Side::from(1), Side::Black);
        assert_eq!(Side::from(2), Side::Both);
    }

    #[test]
    #[should_panic(expected = "Invalid side index: 3")]
    fn test_side_from_usize_panics_on_invalid() {
        let _ = Side::from(3);
    }

    #[test]
    fn test_side_into_usize() {
        assert_eq!(usize::from(Side::White), 0);
        assert_eq!(usize::from(Side::Black), 1);
        assert_eq!(usize::from(Side::Both), 2);
    }
}
