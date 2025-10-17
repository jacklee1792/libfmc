use std::fmt::Display;

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Move {
    U,
    U2,
    U3,
    D,
    D2,
    D3,
    F,
    F2,
    F3,
    B,
    B2,
    B3,
    R,
    R2,
    R3,
    L,
    L2,
    L3,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Move::*;
        match self {
            U => write!(f, "U"),
            U2 => write!(f, "U2"),
            U3 => write!(f, "U'"),
            D => write!(f, "D"),
            D2 => write!(f, "D2"),
            D3 => write!(f, "D'"),
            F => write!(f, "F"),
            F2 => write!(f, "F2"),
            F3 => write!(f, "F'"),
            B => write!(f, "B"),
            B2 => write!(f, "B2"),
            B3 => write!(f, "B'"),
            R => write!(f, "R"),
            R2 => write!(f, "R2"),
            R3 => write!(f, "R'"),
            L => write!(f, "L"),
            L2 => write!(f, "L2"),
            L3 => write!(f, "L'"),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Move::U),
            "U2" => Ok(Move::U2),
            "U'" => Ok(Move::U3),
            "D" => Ok(Move::D),
            "D2" => Ok(Move::D2),
            "D'" => Ok(Move::D3),
            "F" => Ok(Move::F),
            "F2" => Ok(Move::F2),
            "F'" => Ok(Move::F3),
            "B" => Ok(Move::B),
            "B2" => Ok(Move::B2),
            "B'" => Ok(Move::B3),
            "L" => Ok(Move::L),
            "L2" => Ok(Move::L2),
            "L'" => Ok(Move::L3),
            "R" => Ok(Move::R),
            "R2" => Ok(Move::R2),
            "R'" => Ok(Move::R3),
            _ => Err(format!("Invalid move: {value}")),
        }
    }
}

impl Move {
    pub fn all() -> &'static [Move] {
        use Move::*;
        &[
            U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3, R, R2, R3, L, L2, L3,
        ]
    }

    pub fn drud_moveset() -> &'static [Move] {
        use Move::*;
        &[U, U2, U3, D, D2, D3, F2, B2, R2, L2]
    }

    /// The face that this move turns.
    pub fn face(&self) -> Face {
        use Move::*;
        match self {
            U | U2 | U3 => Face::U,
            D | D2 | D3 => Face::D,
            F | F2 | F3 => Face::F,
            B | B2 | B3 => Face::B,
            R | R2 | R3 => Face::R,
            L | L2 | L3 => Face::L,
        }
    }

    /// The face that the move turns, and the number of clockwise turns needed on that face
    /// to achieve the move.
    pub fn decompose(&self) -> (Face, i32) {
        use Move::*;
        let n = match self {
            U | D | F | B | R | L => 1,
            U2 | D2 | F2 | B2 | R2 | L2 => 2,
            U3 | D3 | F3 | B3 | R3 | L3 => 3,
        };
        (self.face(), n)
    }

    pub fn is_half_turn(&self) -> bool {
        use Move::*;
        matches!(self, U2 | D2 | F2 | B2 | R2 | L2)
    }

    pub fn is_clockwise_turn(&self) -> bool {
        use Move::*;
        matches!(self, U | D | F | B | R | L)
    }

    pub fn is_counterclockwise_turn(&self) -> bool {
        use Move::*;
        matches!(self, U3 | D3 | F3 | B3 | R3 | L3)
    }

    pub fn is_quarter_turn(&self) -> bool {
        self.is_clockwise_turn() || self.is_counterclockwise_turn()
    }

    pub fn cancels_with(&self, other: &Self) -> bool {
        self.face() == other.face()
    }

    pub fn commutes_with(&self, other: &Self) -> bool {
        self.face() == other.face() || self.face() == other.face().opposite()
    }
}
