use std::fmt::{self, Display};

use crate::*;

/// An corner on the cube without a specified orientation. Can be used both to refer to
/// a location on the cube, or a particular piece on the cube.
//
// DANGER !! Renumbering is not safe!
// Corners are chosen so that the LSB encodes the HTR-invariant tetrad that
// the corner belongs to: see `Tetrad`.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Corner {
    UBL = 0,
    UBR = 1,
    UFR = 2,
    UFL = 3,
    DFL = 4,
    DFR = 5,
    DBR = 6,
    DBL = 7,
}

impl From<u8> for Corner {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl TryFrom<&str> for Corner {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 3 {
            return Err(());
        }

        let mut chars = s.chars();
        let f1: Face = chars.next().unwrap().try_into().unwrap();
        let f2: Face = chars.next().unwrap().try_into().unwrap();
        let f3: Face = chars.next().unwrap().try_into().unwrap();
        (f1, f2, f3).try_into()
    }
}

impl TryFrom<(Face, Face, Face)> for Corner {
    type Error = ();

    fn try_from(f: (Face, Face, Face)) -> Result<Self, Self::Error> {
        use Corner::*;
        use Face::*;
        let mut f = [f.0, f.1, f.2];
        f.sort();

        match f {
            [U, F, R] => Ok(UFR),
            [U, F, L] => Ok(UFL),
            [U, B, L] => Ok(UBL),
            [U, B, R] => Ok(UBR),
            [D, F, R] => Ok(DFR),
            [D, F, L] => Ok(DFL),
            [D, B, L] => Ok(DBL),
            [D, B, R] => Ok(DBR),
            _ => Err(()),
        }
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Corner {
    pub const fn all() -> [Corner; 8] {
        [
            Corner::from_u8(0),
            Corner::from_u8(1),
            Corner::from_u8(2),
            Corner::from_u8(3),
            Corner::from_u8(4),
            Corner::from_u8(5),
            Corner::from_u8(6),
            Corner::from_u8(7),
        ]
    }

    pub const fn from_u8(x: u8) -> Corner {
        debug_assert!(x < 8);
        unsafe { std::mem::transmute::<u8, Corner>(x) }
    }

    pub const fn tetrad(&self) -> Tetrad {
        Tetrad::from_u8((*self as u8) & 1)
    }
}
