use std::ops::Add;

/// Orientation of an edge.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EO {
    /// The edge is oriented.
    Solved = 0,

    /// The edge is unoriented.
    Flipped = 1,
}

impl Add<Self> for EO {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self as u8 ^ rhs as u8)
    }
}

impl From<u8> for EO {
    fn from(x: u8) -> Self {
        Self::from_u8(x) 
    }
}

impl EO {
    fn inverse(&self) -> Self {
        match self {
            EO::Solved => EO::Flipped,
            EO::Flipped => EO::Solved,
        }
    }

    pub const fn from_u8(x: u8) -> EO {
        debug_assert!(x < 2);
        unsafe { std::mem::transmute::<u8, EO>(x) }
    }
}

