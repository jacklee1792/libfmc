use std::fmt::Display;

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
