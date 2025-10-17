use super::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tetrad {
    A = 0,
    B = 1,
}

impl From<u8> for Tetrad {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl Tetrad {
    const fn corners(&self) -> [Corner; 4] {
        todo!()
    }

    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 8);
        unsafe { std::mem::transmute::<u8, Tetrad>(x) }
    }
}
