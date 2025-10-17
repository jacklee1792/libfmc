use std::fmt::{self, Display};

use crate::*;

/// An edge on the cube without a specified orientation. Can be used both to refer to
/// a location on the cube, or a particular piece on the cube.
//
// DANGER !! Renumbering is not safe!
// Edges are chosen so that bits 2 and 3 encode the slice that the edge belongs
// to: see `Axis`.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Edge {
    // S slice
    UR = 0,
    DR = 1,
    DL = 2,
    UL = 3,
    // M slice
    UF = 4,
    DF = 5,
    DB = 6,
    UB = 7,
    // E slice
    FR = 8,
    BR = 9,
    BL = 10,
    FL = 11,
}

impl From<u8> for Edge {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl From<u8> for Edge {
    fn from(x: u8) -> Self {
        debug_assert!(x < 12);
        unsafe { std::mem::transmute::<u8, Edge>(x) }
    }
}

impl TryFrom<&str> for Edge {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err(());
        }
        let c1: Face = s.chars().nth(0).unwrap().try_into().unwrap();
        let c2: Face = s.chars().nth(1).unwrap().try_into().unwrap();
        (c1, c2).try_into()
    }
}

impl TryFrom<(Face, Face)> for Edge {
    type Error = ();

    fn try_from(f: (Face, Face)) -> Result<Self, Self::Error> {
        use Edge::*;
        use Face::*;
        let mut f = [f.0, f.1];
        f.sort();
        match f {
            [U, F] => Ok(UF),
            [U, L] => Ok(UL),
            [U, B] => Ok(UB),
            [U, R] => Ok(UR),
            [D, F] => Ok(DF),
            [D, L] => Ok(DL),
            [D, B] => Ok(DB),
            [D, R] => Ok(DR),
            [F, R] => Ok(FR),
            [F, L] => Ok(FL),
            [B, L] => Ok(BL),
            [B, R] => Ok(BR),
            _ => Err(()),
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Edge {
    pub const fn slice(&self) -> Slice {
        use Edge::*;
        match self {
            UF | UB | DF | DB => Slice::M,
            UL | UR | DL | DR => Slice::S,
            FL | FR | BL | BR => Slice::E,
        }
    }

    pub const fn all() -> [Edge; 12] {
        use Edge::*;
        [UF, UL, UB, UR, DF, DL, DB, DR, FR, FL, BL, BR]
    }
}
