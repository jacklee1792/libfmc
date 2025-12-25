use std::fmt::Display;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rotation {
    X,
    X2,
    X3,
    Y,
    Y2,
    Y3,
    Z,
    Z2,
    Z3,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Rotation::*;
        match self {
            X => write!(f, "x"),
            X2 => write!(f, "x2"), 
            X3 => write!(f, "x'"),
            Y => write!(f, "y"),
            Y2 => write!(f, "y2"),
            Y3 => write!(f, "y'"),
            Z => write!(f, "z"),
            Z2 => write!(f, "z2"),
            Z3 => write!(f, "z'"),
        }
    }
}

impl TryFrom<&str> for Rotation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x"  => Ok(Rotation::X),
            "x2" => Ok(Rotation::X2),
            "x'" => Ok(Rotation::X3),
            "y"  => Ok(Rotation::Y),
            "y2" => Ok(Rotation::Y2),
            "y'" => Ok(Rotation::Y3),
            "z"  => Ok(Rotation::Z),
            "z2" => Ok(Rotation::Z2),
            "z'" => Ok(Rotation::Z3),
            other => Err(format!("Invalid rotation: {other}")),
        }
    }
}

impl From<u8> for Rotation {
    fn from(value: u8) -> Self {  
        Self::from_u8(value)
    }
}

impl From<Rotation> for u8 {
    fn from(value: Rotation) -> Self {
       value as u8 
    }
}

impl Rotation {
    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 9);
        unsafe { std::mem::transmute::<u8, Rotation>(x) }
    }
}
