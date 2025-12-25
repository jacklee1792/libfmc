use crate::*;
use std::fmt::Display;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SliceMove {
    M,
    M2,
    M3,
    E,
    E2,
    E3,
    S,
    S2,
    S3,
}

impl Display for SliceMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SliceMove::*;
        match self {
            M => write!(f, "M"),
            M2 => write!(f, "M2"),
            M3 => write!(f, "M'"),
            E => write!(f, "E"),
            E2 => write!(f, "E2"),
            E3 => write!(f, "E'"),
            S => write!(f, "S"),
            S2 => write!(f, "S2"),
            S3 => write!(f, "S'"),
        }
    }
}

impl TryFrom<&str> for SliceMove {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use SliceMove::*;
        match value {
            "M" => Ok(M),
            "M2" => Ok(M2),
            "M'" => Ok(M3),
            "E" => Ok(E),
            "E2" => Ok(E2),
            "E'" => Ok(E3),
            "S" => Ok(S),
            "S2" => Ok(S2),
            "S'" => Ok(S3),
            other => Err(format!("Invalid slice move: {other}")),
        }
    }
}

impl From<u8> for SliceMove {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<SliceMove> for u8 {
    fn from(value: SliceMove) -> Self {
        value as u8
    }
}

impl SliceMove {
    pub fn all() -> [SliceMove; 9] {
        use SliceMove::*;
        [M, M2, M3, E, E2, E3, S, S2, S3]
    }

    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 9);
        unsafe { std::mem::transmute::<u8, SliceMove>(x) }
    }

    pub fn slice(self) -> Slice {
        use SliceMove::*;
        match self {
            M | M2 | M3 => Slice::M,
            E | E2 | E3 => Slice::E,
            S | S2 | S3 => Slice::S,
        }
    }

    pub fn decompose(self) -> (Slice, i32) {
        use SliceMove::*;
        let n = match self {
            M | E | S => 1,
            M2 | E2 | S2 => 2,
            M3 | E3 | S3 => 3,
        };
        (self.slice(), n)
    }
}
