use std::fmt::Display;
use std::fmt::Debug;
use super::*;

// DANGER !! Not safe to renumber, same as `Axis`
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Slice {
    S = 0,
    M = 1,
    E = 2,
}

impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<u8> for Slice {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl Slice {
    const fn edges(self) -> [Edge; 4] {
        use Edge::*;
        match self {
            Slice::S => [UL, UR, DL, DR],
            Slice::M => [UF, UB, DF, DB],
            Slice::E => [FL, FR, BL, BR],
        }
    }

    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 8);
        unsafe { std::mem::transmute::<u8, Slice>(x) }
    }

    /// The axis that the slice runs perpendicular to.
    pub fn perp(self) -> Axis {
        Axis::from_u8(self as u8)
    }
}
