use std::fmt::Display;
use std::ops::Add;

use crate::*;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Move {
    // Face turns
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

impl From<u8> for Move {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<Move> for u8 {
    fn from(value: Move) -> Self {
        value as u8
    }
}

impl Add<Self> for Move {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let (m1, n1) = self.decompose();
        let (m2, n2) = rhs.decompose();
        if m1 != m2 {
            return None;
        }
        let n = (n1 + n2) % 4;
        match (m1, n) {
            (Face::F, 1) => Some(Move::F),
            (Face::F, 2) => Some(Move::F2),
            (Face::F, 3) => Some(Move::F3),
            (Face::B, 1) => Some(Move::B),
            (Face::B, 2) => Some(Move::B2),
            (Face::B, 3) => Some(Move::B3),
            (Face::L, 1) => Some(Move::L),
            (Face::L, 2) => Some(Move::L2),
            (Face::L, 3) => Some(Move::L3),
            (Face::R, 1) => Some(Move::R),
            (Face::R, 2) => Some(Move::R2),
            (Face::R, 3) => Some(Move::R3),
            (Face::U, 1) => Some(Move::U),
            (Face::U, 2) => Some(Move::U2),
            (Face::U, 3) => Some(Move::U3),
            (Face::D, 1) => Some(Move::D),
            (Face::D, 2) => Some(Move::D2),
            (Face::D, 3) => Some(Move::D3),
            (_, _) => None,
        }
    }
}

impl Move {
    pub const fn all() -> [Move; 18] {
        use Move::*;
        [
            U, U2, U3, D, D2, D3, F, F2, F3, B, B2, B3, R, R2, R3, L, L2, L3,
        ]
    }

    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 18);
        unsafe { std::mem::transmute::<u8, Move>(x) }
    }

    pub fn drud_moveset() -> [Move; 10] {
        use Move::*;
        [U, U2, U3, D, D2, D3, F2, B2, R2, L2]
    }

    pub fn htr_moveset() -> [Move; 6] {
        use Move::*;
        [U2, D2, F2, B2, R2, L2]
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

    /// The axis perpendicular to the face that this move turns.
    pub fn axis(self) -> Axis {
        self.face().axis()
    }

    /// The face that the move turns, and the number of clockwise turns needed on that face
    /// to achieve the move.
    pub fn decompose(self) -> (Face, i32) {
        use Move::*;
        let n = match self {
            U | D | F | B | R | L => 1,
            U2 | D2 | F2 | B2 | R2 | L2 => 2,
            U3 | D3 | F3 | B3 | R3 | L3 => 3,
        };
        (self.face(), n)
    }

    pub fn direction(self) -> Direction {
        use Move::*;
        match self {
            U | D | F | B | R | L => Direction::Clockwise,
            U2 | D2 | F2 | B2 | R2 | L2 => Direction::Double,
            U3 | D3 | F3 | B3 | R3 | L3 => Direction::Counterclockwise,
        }
    }

    pub fn is_half_turn(self) -> bool {
        use Move::*;
        matches!(self, U2 | D2 | F2 | B2 | R2 | L2)
    }

    pub fn is_clockwise(self) -> bool {
        use Move::*;
        matches!(self, U | D | F | B | R | L)
    }

    pub fn is_counterclockwise_turn(self) -> bool {
        use Move::*;
        matches!(self, U3 | D3 | F3 | B3 | R3 | L3)
    }

    pub fn is_quarter_turn(self) -> bool {
        self.is_clockwise() || self.is_counterclockwise_turn()
    }

    pub fn cancels_with(self, other: Self) -> bool {
        self.face() == other.face()
    }

    pub fn commutes_with(self, other: Self) -> bool {
        self.face() == other.face() || self.face() == other.face().opposite()
    }

    pub fn canonically_succeeds(self, prev: Self) -> bool {
        !self.commutes_with(prev) || (!self.cancels_with(prev) && self > prev)
    }

    pub fn canonically_precedes(self, next: Self) -> bool {
        next.canonically_succeeds(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_canonical() {
        use Move::*;
        assert_eq!(D.canonically_succeeds(U), true);
        assert_eq!(U.canonically_succeeds(D), false);
    }
}
