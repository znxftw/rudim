use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SideMap<T>(pub [T; 2]);

impl<T> SideMap<T> {
    #[inline(always)]
    pub const fn new(white: T, black: T) -> Self {
        Self([white, black])
    }
}

impl<T> Index<Side> for SideMap<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::White => &self.0[0],
            Side::Black => &self.0[1],
            Side::Both => panic!("Cannot index SideMap with Side::Both"),
        }
    }
}

impl<T> IndexMut<Side> for SideMap<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::White => &mut self.0[0],
            Side::Black => &mut self.0[1],
            Side::Both => panic!("Cannot index SideMap with Side::Both"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Side {
    White = 0,
    Black,
    Both,
}

impl Side {
    pub fn other(self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
            Side::Both => Side::Both,
        }
    }
}

impl From<usize> for Side {
    fn from(value: usize) -> Self {
        match value {
            0 => Side::White,
            1 => Side::Black,
            2 => Side::Both,
            _ => panic!("Invalid side index: {}", value),
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
