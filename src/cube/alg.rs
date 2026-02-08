use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use crate::*;

use super::*;

#[derive(Clone)]
pub struct Alg(Vec<Move>);

#[macro_export]
macro_rules! alg {
    ( $( $m:expr ),* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut v = Vec::new();
        $(
            v.push($m);
        )*
        Alg::from(v)
    }};
}

impl From<Vec<Move>> for Alg {
    fn from(alg: Vec<Move>) -> Self {
        Alg(alg)
    }
}

impl FromIterator<Move> for Alg {
    fn from_iter<T: IntoIterator<Item = Move>>(moves: T) -> Self {
        Alg(Vec::from_iter(moves))
    }
}

impl Debug for Alg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(Move::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Display for Alg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl TryFrom<&str> for Alg {
    type Error = String;

    fn try_from(mut s: &str) -> Result<Self, Self::Error> {
        let mut moves = Vec::new();
        loop {
            s = s.trim_start();
            if s.is_empty() {
                break;
            }
            let mut found = false;
            if s.len() >= 2 {
                let (hd, tl) = s.split_at(2);
                if let Ok(m) = Move::try_from(hd) {
                    s = tl;
                    moves.push(m);
                    found = true;
                }
            }
            if !found {
                let (hd, tl) = s.split_at(1);
                s = tl;
                let m = Move::try_from(hd)?;
                moves.push(m);
            }
        }
        Ok(Self(moves))
    }
}

impl Alg {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, m: Move) {
        self.0.push(m)
    }

    pub fn iter(&self) -> impl Iterator<Item = Move> {
        self.0.iter().map(|m| *m)
    }

    pub fn into_iter(self) -> impl Iterator<Item = Move> {
        self.0.into_iter()
    }

    pub fn is_canonical(&self) -> bool {
        self.0.windows(2).all(|w| w[0].canonically_precedes(w[1]))
    }

    pub fn canonicalize(&self) -> Alg {
        struct Item {
            axis: Axis,
            primary: Direction,
            secondary: Direction,
        }

        let mut stack: Vec<Item> = Vec::new();
        for m in self.0.iter() {
            let (p, s) = if m.face() == m.axis().primary() {
                (m.direction(), Direction::Zero)
            } else {
                (Direction::Zero, m.direction())
            };
            match stack.last_mut() {
                Some(Item {
                    axis,
                    primary,
                    secondary,
                }) if *axis == m.axis() => {
                    *primary += p;
                    *secondary += s;
                    if *primary == Direction::Zero && *secondary == Direction::Zero {
                        stack.pop();
                    }
                }
                _ => {
                    stack.push(Item {
                        axis: m.axis(),
                        primary: p,
                        secondary: s,
                    });
                }
            }
        }

        let mut out = Vec::new();
        for Item {
            axis,
            primary,
            secondary,
        } in stack
        {
            if let Some(m) = axis.primary().turn(primary) {
                out.push(m);
            }
            if let Some(m) = axis.secondary().turn(secondary) {
                out.push(m);
            }
        }
        Alg(out)
    }
}

#[cfg(test)]
mod tests {
    use super::Alg;
    use crate::*;

    #[test]
    fn test_canonicalize() {
        let alg = Alg::try_from("R D U").unwrap();
        let s = format!("{}", alg.canonicalize());
        expect!(s, "R U D");

        let alg = Alg::try_from("R D U D").unwrap();
        let s = format!("{}", alg.canonicalize());
        expect!(s, "R U D2");

        let alg = Alg::try_from("B R D D2 D R' F").unwrap();
        let s = format!("{}", alg.canonicalize());
        expect!(s, "F B");
    }

    #[test]
    fn test_parse_ok() {
        let algs = vec![
            "",
            "R",
            "RUFUDL",
            "  R'U'F'U'D'      L'F2R2   B     ",
            "R' U' F B2 L2 U2 R2 U' R2 B2 D B2 R2 U' R' F R D2 R F2 D2 R F' L R' U' F",
        ];
        for alg in algs {
            assert!(Alg::try_from(alg).is_ok());
        }
    }

    #[test]
    fn test_parse_fail() {
        let algs = vec!["B3", "U '", "rF"];
        for alg in algs {
            assert!(Alg::try_from(alg).is_err());
        }
    }
}
