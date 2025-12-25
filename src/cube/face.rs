use std::fmt::Display;
use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Face {
    U,
    D,
    F,
    B,
    R,
    L,
}

impl Face {
    pub fn opposite(&self) -> Self {
        use Face::*;
        match self {
            U => D,
            D => U,
            F => B,
            B => F,
            R => L,
            L => R,
        }
    }

    /// The axis perpendicular to this face.
    pub fn axis(self) -> Axis {
        use Face::*;
        match self {
            U | D => Axis::UD,
            F | B => Axis::FB,
            R | L => Axis::LR,
        }
    }

    pub fn turn(self, d: Direction) -> Option<Move> {
        use Direction::*;
        match (self, d) {
            (_, Zero) => None,
            (Face::F, Clockwise) => Some(Move::F),
            (Face::F, Double) => Some(Move::F2),
            (Face::F, Counterclockwise) => Some(Move::F3),
            (Face::B, Clockwise) => Some(Move::B),
            (Face::B, Double) => Some(Move::B2),
            (Face::B, Counterclockwise) => Some(Move::B3),
            (Face::L, Clockwise) => Some(Move::L),
            (Face::L, Double) => Some(Move::L2),
            (Face::L, Counterclockwise) => Some(Move::L3),
            (Face::R, Clockwise) => Some(Move::R),
            (Face::R, Double) => Some(Move::R2),
            (Face::R, Counterclockwise) => Some(Move::R3),
            (Face::U, Clockwise) => Some(Move::U),
            (Face::U, Double) => Some(Move::U2),
            (Face::U, Counterclockwise) => Some(Move::U3),
            (Face::D, Clockwise) => Some(Move::D),
            (Face::D, Double) => Some(Move::D2),
            (Face::D, Counterclockwise) => Some(Move::D3),
        }
    }
}

impl TryFrom<&str> for Face {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 1 {
            Err(())
        } else {
            s.chars().next().unwrap().try_into()
        }
    }
}

impl TryFrom<char> for Face {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Face::*;
        match c.to_ascii_uppercase() {
            'U' => Ok(U),
            'D' => Ok(D),
            'F' => Ok(F),
            'B' => Ok(B),
            'R' => Ok(R),
            'L' => Ok(L),
            _ => Err(()),
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
