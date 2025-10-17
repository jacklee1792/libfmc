// use std::{
//     fmt::{self, Display},
//     ops::Add,
//     simd::u8x16,
// };

// use super::*;

// /// Edges of the cube.
// #[repr(transparent)]
// #[derive(Copy, Clone, Debug)]
// pub struct Edges(u8x16);

// // Bits 0..3 in the n-th lane describe which piece is in position n.
// const EP_MASK: u8x16 = u8x16::splat(0b0000_1111);

// // Bits 4, 5, and 6 correspond to edge orientation with respect to FB, LR, and UD, respectively
// // following the permutation.
// const EO_MASK: u8x16 = u8x16::splat(0b0111_0000);
// const EOFB_MASK: u8x16 = u8x16::splat(0b0001_0000);
// const EOLR_MASK: u8x16 = u8x16::splat(0b0010_0000);
// const EOUD_MASK: u8x16 = u8x16::splat(0b0100_0000);

// impl Default for Edges {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Add<Self> for Edges {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.compose(rhs)
//     }
// }

// impl Add<Move> for Edges {
//     type Output = Self;

//     fn add(self, rhs: Move) -> Self::Output {
//         self.apply_move(rhs)
//     }
// }

// impl Add<Alg> for Edges {
//     type Output = Self;

//     fn add(self, rhs: Alg) -> Self::Output {
//         self.apply_alg(&rhs)
//     }
// }

// impl Edges {
//     pub const fn new() -> Self {
//         Self::from_magic(0x000102030405060708090a0b0c0d0e0f)
//     }

//     pub const fn from_magic(m: u128) -> Self {
//         Self(u8x16::from_array(m.to_be_bytes()))
//     }

//     pub fn magic(&self) -> String {
//         format!("0x{:032x}", u128::from_be_bytes(*self.0.as_array()))
//     }

//     /// The cube the the same EP, but the given EO instead.
//     fn with_eo(&self, eo: u8x16) -> Self {
//         let m = self.0 & EO_MASK;
//         Self(self.0 ^ m | eo)
//     }

//     pub fn compose(&self, rhs: Self) -> Self {
//         let ret = self.0.swizzle_dyn(rhs.0 & EP_MASK);
//         Self(ret ^ (rhs.0 & EO_MASK))
//     }

//     pub fn inverse(&self) -> Self {
//         // Any EP raised to the 27720th power is the identity, since
//         // LCM(1..=12) = 27720 (consider disjoint cycles of EP). Thus to find the inverse
//         // EP, we can take it to the 27719th power.
//         let f = |x: Self, y: Self| Self(x.0.swizzle_dyn(y.0 & EP_MASK));
//         let s1 = Self(self.0 & EP_MASK);
//         let s3 = f(s1, f(s1, s1));
//         let s6 = f(s3, s3);
//         let s12 = f(s6, s6);
//         let s24 = f(s12, s12);
//         let s27 = f(s24, s3);
//         let s54 = f(s27, s27);
//         let s108 = f(s54, s54);
//         let s216 = f(s108, s108);
//         let s432 = f(s216, s216);
//         let s433 = f(s432, s1);
//         let s866 = f(s433, s433);
//         let s1732 = f(s866, s866);
//         let s3464 = f(s1732, s1732);
//         let s6928 = f(s3464, s3464);
//         let s13856 = f(s6928, s6928);
//         let s13859 = f(s13856, s3);
//         let s27718 = f(s13859, s13859);
//         let s27719 = f(s27718, s1);

//         Edges::new().with_eo(self.0 & EO_MASK).compose(s27719)
//     }

//     /// The edge at the given position.
//     pub fn at(&self, e: Edge) -> Edge {
//         self.0.as_array()[e as usize].into()
//     }

//     pub fn apply_move(&self, m: Move) -> Self {
//         use Move::*;
//         let c = match m {
//             U => Edges::from_magic(0x4801024b0405060743090a400c0d0e0f),
//             U2 => Edges::from_magic(0x03010200040506070b090a080c0d0e0f),
//             U3 => Edges::from_magic(0x4b0102480405060740090a430c0d0e0f),
//             D => Edges::from_magic(0x004a4903040506070841420b0c0d0e0f),
//             D2 => Edges::from_magic(0x0002010304050607080a090b0c0d0e0f),
//             D3 => Edges::from_magic(0x00494a03040506070842410b0c0d0e0f),
//             F => Edges::from_magic(0x171402031005061108090a0b0c0d0e0f),
//             F2 => Edges::from_magic(0x010002030705060408090a0b0c0d0e0f),
//             F3 => Edges::from_magic(0x141702031105061008090a0b0c0d0e0f),
//             B => Edges::from_magic(0x000116150412130708090a0b0c0d0e0f),
//             B2 => Edges::from_magic(0x000103020406050708090a0b0c0d0e0f),
//             B3 => Edges::from_magic(0x000115160413120708090a0b0c0d0e0f),
//             R => Edges::from_magic(0x000102032928060724250a0b0c0d0e0f),
//             R2 => Edges::from_magic(0x000102030504060709080a0b0c0d0e0f),
//             R3 => Edges::from_magic(0x000102032829060725240a0b0c0d0e0f),
//             L => Edges::from_magic(0x0001020304052a2b080927260c0d0e0f),
//             L2 => Edges::from_magic(0x000102030405070608090b0a0c0d0e0f),
//             L3 => Edges::from_magic(0x0001020304052b2a080926270c0d0e0f),
//         };
//         self.compose(c)
//     }

//     pub fn apply_alg(&self, a: &Alg) -> Self {
//         a.iter().fold(*self, |acc, m| acc.apply_move(*m))
//     }

//     pub fn dbg(&self) {
//         let edges = (0..12).map(Edge::from).collect::<Vec<_>>();

//         let ep = (self.0 & EP_MASK)
//             .as_array()
//             .iter()
//             .take(12)
//             .map(|e| Edge::from(*e))
//             .collect::<Vec<_>>();

//         let eofb = ((self.0 & EOFB_MASK) >> 4)
//             .as_array()
//             .iter()
//             .map(|&x| x as usize)
//             .take(12)
//             .collect::<Vec<_>>();

//         let eolr = ((self.0 & EOLR_MASK) >> 5)
//             .as_array()
//             .iter()
//             .map(|&x| x as usize)
//             .take(12)
//             .collect::<Vec<_>>();

//         let eoud = ((self.0 & EOUD_MASK) >> 6)
//             .as_array()
//             .iter()
//             .map(|&x| x as usize)
//             .take(12)
//             .collect::<Vec<_>>();

//         // Print aligned
//         print!("{:6}", "");
//         for e in &edges {
//             print!("{:<4}", format!("{:?}", e));
//         }
//         println!();

//         print!("{:6}", "ep:");
//         for e in &ep {
//             print!("{:<4}", format!("{:?}", e));
//         }
//         println!();

//         print!("{:6}", "eofb:");
//         for v in &eofb {
//             print!("{:<4}", v);
//         }
//         println!();

//         print!("{:6}", "eolr:");
//         for v in &eolr {
//             print!("{:<4}", v);
//         }
//         println!();

//         print!("{:6}", "eoud:");
//         for v in &eoud {
//             print!("{:<4}", v);
//         }
//         println!();
//     }
// }

// /// An edge on the cube without a specified orientation. Can be used both to refer to
// /// a location on the cube, or a particular piece on the cube.
// //
// // DANGER !! Renumbering is not safe!
// // Edges are chosen so that masking with 0xf gives the slice the edge belongs to.
// #[repr(u8)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Edge {
//     // M slice
//     UF = 0,
//     DF = 1,
//     DB = 2,
//     UB = 3,
//     // E slice
//     FR = 4,
//     BR = 5,
//     BL = 6,
//     FL = 7,
//     // S slice
//     UR = 8,
//     DR = 9,
//     DL = 10,
//     UL = 11,
// }

// impl From<u8> for Edge {
//     fn from(x: u8) -> Self {
//         debug_assert!(x < 12);
//         unsafe { std::mem::transmute::<u8, Edge>(x) }
//     }
// }

// impl TryFrom<&str> for Edge {
//     type Error = ();

//     fn try_from(s: &str) -> Result<Self, Self::Error> {
//         if s.len() != 2 {
//             return Err(());
//         }
//         let c1: Face = s.chars().nth(0).unwrap().try_into().unwrap();
//         let c2: Face = s.chars().nth(1).unwrap().try_into().unwrap();
//         (c1, c2).try_into()
//     }
// }

// impl TryFrom<(Face, Face)> for Edge {
//     type Error = ();

//     fn try_from(f: (Face, Face)) -> Result<Self, Self::Error> {
//         use Edge::*;
//         use Face::*;
//         let mut f = [f.0, f.1];
//         f.sort();
//         match f {
//             [U, F] => Ok(UF),
//             [U, L] => Ok(UL),
//             [U, B] => Ok(UB),
//             [U, R] => Ok(UR),
//             [D, F] => Ok(DF),
//             [D, L] => Ok(DL),
//             [D, B] => Ok(DB),
//             [D, R] => Ok(DR),
//             [F, R] => Ok(FR),
//             [F, L] => Ok(FL),
//             [B, L] => Ok(BL),
//             [B, R] => Ok(BR),
//             _ => Err(()),
//         }
//     }
// }

// impl Display for Edge {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Edge {
//     pub const fn slice(&self) -> Slice {
//         use Edge::*;
//         match self {
//             UF | UB | DF | DB => Slice::M,
//             UL | UR | DL | DR => Slice::S,
//             FL | FR | BL | BR => Slice::E,
//         }
//     }

//     pub const fn all() -> [Edge; 12] {
//         use Edge::*;
//         [UF, UL, UB, UR, DF, DL, DB, DR, FR, FL, BL, BR]
//     }
// }

// /// Axis-agnostic orientation of an edge.
// ///
// // DANGER !! Renumbering is not safe!
// #[repr(u8)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum EO {
//     Oriented = 0,
//     Misoriented = 1,
// }

// impl From<u8> for EO {
//     fn from(x: u8) -> Self {
//         debug_assert!(x < 2);
//         unsafe { std::mem::transmute::<u8, EO>(x) }
//     }
// }

// impl EO {
//     const fn flip(self) -> Self {
//         let x = self as u8 ^ 1;
//         unsafe { std::mem::transmute::<u8, EO>(x) }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::simd::u8x8;

//     use crate::{Edges, cube::edges_old::EO_MASK};

//     #[test]
//     fn dbg() {
//         use crate::Alg;
//         use crate::Move;
//         let alg = Alg::try_from(
//             "R' U' F B2 L2 U2 R2 U' R2 B2 D B2 R2 U' R' F R D2 R F2 D2 R F' L R' U' F",
//         )
//         .unwrap();
//         let c = Edges::default() + alg;
//         c.dbg();
//         (c + c.inverse()).dbg();
//     }
// }

// // A: EP EO
// // A^-1: EO^-1 EP^-1
