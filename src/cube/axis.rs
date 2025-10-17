use super::*;

// DANGER !! Not safe to renumber, same as `Slice`
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    FB = 0,
    LR = 1,
    UD = 2,
}

impl From<u8> for Axis {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl Axis {
    /// The slice that this axis runs perpendicular to.
    const fn slice(self) -> Slice {
        Slice::from_u8(self as u8)
    }

    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 8);
        unsafe { std::mem::transmute::<u8, Axis>(x) }
    }
}
