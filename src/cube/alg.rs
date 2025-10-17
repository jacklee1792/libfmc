use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use super::*;

#[derive(Clone)]
pub struct Alg(Vec<Move>);

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
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Move> {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Alg;

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
