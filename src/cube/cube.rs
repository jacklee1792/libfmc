use super::*;
use std::ops::Add;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Cube {
    pub edges: Edges,
    pub corners: Corners,
}

impl Add<Self> for Cube {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl From<Alg> for Cube {
    fn from(alg: Alg) -> Self {
        Self::default().apply_alg(alg)
    }
}

impl From<Edges> for Cube {
    fn from(edges: Edges) -> Self {
        Cube {
            edges,
            corners: Corners::default(),
        }
    }
}

impl From<Corners> for Cube {
    fn from(corners: Corners) -> Self {
        Cube {
            corners,
            edges: Edges::default(),
        }
    }
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_move(m: Move) -> Self {
        Self::new().apply_move(m)
    }

    pub fn from_alg<A>(a: A) -> Self
    where
        A: Into<Alg>,
    {
        Self::new().apply_alg(a.into())
    }

    pub fn apply_move(self, m: Move) -> Self {
        Cube {
            edges: self.edges.apply_move(m),
            corners: self.corners.apply_move(m),
        }
    }

    pub fn apply_alg(self, a: Alg) -> Self {
        Cube {
            edges: self.edges.apply_alg(&a),
            corners: self.corners.apply_alg(&a),
        }
    }

    pub fn apply_sym(self, s: Sym) -> Self {
        Cube {
            edges: self.edges.apply_sym(s),
            corners: self.corners.apply_sym(s),
        }
    }

    pub fn conjugate_sym(self, s: Sym) -> Self {
        Cube {
            edges: self.edges.conjugate_sym(s),
            corners: self.corners.conjugate_sym(s),
        }
    }

    pub fn compose(self, c: Cube) -> Self {
        Cube {
            edges: self.edges.compose(c.edges),
            corners: self.corners.compose(c.corners),
        }
    }

    pub fn is_solved(self) -> bool {
        self.edges.is_solved() && self.corners.is_solved()
    }

    pub fn is_drud(self) -> bool {
        self.corners.is_drud() && self.edges.is_drud()
    }

    pub fn is_eofb(self) -> bool {
        self.edges.is_eofb()
    }

    pub fn is_eolr(self) -> bool {
        self.edges.is_eolr()
    }

    pub fn is_eoud(self) -> bool {
        self.edges.is_eoud()
    }

    pub fn inverse(self) -> Self {
        Self {
            edges: self.edges.inverse(),
            corners: self.corners.inverse(),
        }
    }
}

use std::fmt::Debug;

impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edges\n{:?}\nCorners\n{:?}", self.edges, self.corners)
    }
}

#[cfg(test)]
mod tests {}
