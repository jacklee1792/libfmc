use std::ops::Add;

/// Orientation of a corner.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CO {
    /// The corner is oriented.
    Solved = 0,

    /// The corner is twisted counterclockwise from the solved orientation.
    CCW = 1,

    /// The corner is twisted clockwise from the solved orientation.
    CW = 2,
}

impl Add<Self> for CO {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self as u8 + rhs as u8;
        Self::from(sum % 3)
    }
}

impl From<u8> for CO {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl CO {
    pub fn inverse(&self) -> Self {
        match self {
            CO::Solved => CO::Solved,
            CO::CCW => CO::CW,
            CO::CW => CO::CCW,
        }
    }

    pub const fn from_u8(x: u8) -> CO {
        debug_assert!(x < 3);
        unsafe { std::mem::transmute::<u8, CO>(x) }
    }
}
