use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
pub enum Direction {
    Zero = 0,
    Clockwise = 1,
    Double = 2,
    Counterclockwise = 3,
}

impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output { 
        Self::from_u8(((self as u8) + (rhs as u8)) % 4)
    }
}

impl Sub<Direction> for Direction {
    type Output = Direction;

    fn sub(self, rhs: Direction) -> Self::Output {
        self + rhs.inverse()
    }
}

impl Direction {
    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 4);
        unsafe { std::mem::transmute::<u8, Direction>(x) }
    }

    pub const fn inverse(self) -> Self {
        use Direction::*;
        match self {
            Zero => Zero,
            Clockwise => Counterclockwise,
            Counterclockwise => Clockwise,
            Double => Double,
        }
    }

    pub fn repeat(self, n: usize) -> Self {
        Self::from_u8((self as usize * n) as u8)
    }
}
