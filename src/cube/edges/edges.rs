use std::simd::u8x16;
use std::simd::cmp::SimdPartialEq;
use std::fmt::Debug;
use std::ops::Add;
use std::fmt;
use crate::{Alg, Move, Edge};
use crate::cube::edges::ops;

/// Edges of the cube.
///
/// ### Representation
/// Edge ordering:
///   UR DR DL UL UF DF DB UB FR BR BL FL
///
/// The underlying representation is a SIMD vector of 16 lanes,
/// each lane representing a edge slot.
/// 
/// Each lane has a packed 8-bit representation, where:
/// - Bits 0..=3 represent the edge piece at that position (0..=16).
/// - Bits 4, 5, and 6 represents the orientation of the edge piece with respect to the FB, LR,
///   and UD axes respectively.
///   - 0 means the edge is oriented;
///   - 1 means the edge is misoriented.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Edges(pub(super) u8x16);

impl Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo!")
    }
}

impl Default for Edges {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<Self> for Edges {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl Add<Move> for Edges {
    type Output = Self;

    fn add(self, rhs: Move) -> Self::Output {
        self.apply_move(rhs)
    }
}

impl Add<Alg> for Edges {
    type Output = Self;

    fn add(self, rhs: Alg) -> Self::Output {
        self.apply_alg(&rhs)
    }
}

impl Edges {
    /// Construct a new `Corners` instance with the identity CP and CO.
    pub const fn new() -> Self {
        Self(u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]))
    }

    /// The slot where the given `Edge` currently resides.
    pub fn find(self, edge: Edge) -> Edge {
        let edge = u8x16::splat(edge as u8);
        let slot = ops::ep(self.0).simd_eq(edge).first_set().unwrap() as u8;
        slot.into()
    }

    pub const fn from_magic(m: u128) -> Self {
        Self(u8x16::from_array(m.to_le_bytes()))
    }
    
    /// Modify EP by specifying for each location which corner populates it. Preserves EO on FB.
    pub fn set_ep_passive<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(Edge) -> Edge,
    {
        let mut a = self.0.to_array();
        for (i, lane) in a.iter_mut().enumerate() {
            let slot = Edge::from(i as u8);
            let piece = f(slot);
            ops::lane_set_ep(a, i, piece as u8);
        }
        Self(u8x8::from_array(a))
    }

    /// Modify CP by specifying for each corner where it's located. Preserves CO on UD.
    pub fn set_cp_active<F>(&self, mut f: F) -> Self
    where
        F: FnMut(Corner) -> Corner,
    {
        let mut a = self.0.to_array();
        for i in 0..8 {
            let src = Corner::from(i as u8);
            let dest = f(src) as usize;
            let lane = &mut a[dest];
            *lane = (*lane & CO_LANE_MASK) | (src as u8);
        }
        Self(u8x8::from_array(a))
    }

    pub fn cycle_dr<I>(cycle: I, axis: Axis) -> Self
    where I: IntoIterator<Item = Corner>
    {
        match axis {
            Axis::UD => Self::cycle_drud(cycle),
            Axis::FB => Self::cycle_drfb(cycle),
            Axis::LR => Self::cycle_drlr(cycle),
        }
    }

    /// A cycle which preserves CO on UD. The first slot in the cycle is sent to the second
    /// slot in the cycle, and so on with the last slot in the cycle being sent to the first slot
    /// in the cycle.
    pub fn cycle_drud<I>(cycle: I) -> Self
    where I: IntoIterator<Item = Corner>
    {
        let cycle = cycle.into_iter().collect::<Vec<_>>();
        let mut cp = CORNERS_IDENT;
        let n = cycle.len();
        for i in 0..n {
            let src = cycle[i] as u8;
            let dst = cycle[(i + 1) % n] as usize;
            cp.as_mut_array()[dst] = src;
        }
        Self(cp)
    }

    /// A cycle which preserves CO on FB. The first slot in the cycle is sent to the second
    /// slot in the cycle, and so on with the last slot in the cycle being sent to the first slot
    /// in the cycle.
    pub fn cycle_drfb<I>(cycle: I) -> Self
    where I: IntoIterator<Item = Corner> {
        let ret = ops::set_cofb(Self::cycle_drud(cycle).0, u8x8::splat(0));
        Self(ret)
    }
    
    /// A cycle which preserves CO on LR. The first slot in the cycle is sent to the second
    /// slot in the cycle, and so on with the last slot in the cycle being sent to the first slot
    /// in the cycle.
    pub fn cycle_drlr<I>(cycle: I) -> Self
    where I: IntoIterator<Item = Corner> {
        let ret = ops::set_colr(Self::cycle_drud(cycle).0, u8x8::splat(0));
        Self(ret)
    }

    pub fn compose(self, rhs: Self) -> Self {
        let ret = self.0.swizzle_dyn(rhs.0 & CP_MASK) + (rhs.0 & CO_MASK);
        Self(ret.simd_min(ret - CO_MASK))
    }

    pub fn inverse(self) -> Self {
        // Any CP raised to the 840th power is the identity, since
        // LCM(1..=8) = 840 (consider disjoint cycles of CP). Thus to find the inverse
        // CP, we can take it to the 839th power.
        let inv_cp = {
            let f = |x: Self, y: Self| Self(x.0.swizzle_dyn(ops::cp(y.0)));
            let s1 = Self(self.0 & CP_MASK);
            let s2 = f(s1, s1);
            let s4 = f(s2, s2);
            let s8 = f(s4, s4);
            let s16 = f(s8, s8);
            let s32 = f(s16, s16);
            let s64 = f(s32, s32);
            let s65 = f(s64, s1);
            let s129 = f(s64, s65);
            let s258 = f(s129, s129);
            let s516 = f(s258, s258);
            let s774 = f(s516, s258);
            let s839 = f(s774, s65);
            s839
        };

        let inv_coud = ops::invmod3(ops::coud(self.0));
        Self(ops::cons(inv_coud, ops::CP_IDENT)).compose(inv_cp)
    }

    pub fn apply_move(self, m: Move) -> Self {
        // // use Face::*;
        // // use Corner::*;
        // let (f, n) = m.decompose();
        // let c = match f {
        //     U => Corners::cycle_drud([UFR, UFL, UBL, UBR]),
        //     F => Corners::cycle_drfb([UFR, DFR, DFL, UFL]),
        //     R => Corners::cycle_drlr([UFR, UBR, DBR, DFR]),
        //     D => Corners::cycle_drud([DFL, DFR, DBR, DBL]),
        //     B => Corners::cycle_drfb([UBR, UBL, DBL, DBR]),
        //     L => Corners::cycle_drlr([UFL, DFL, DBL, UBL]),
        // };
        // let mut ret = self;
        // for _ in 0..n {
        //     ret = ret.compose(c);
        // }
        // ret
        // use Move::*;
        // let c = match m {
        //     U => Edges::from_magic(0x4801024b0405060743090a400c0d0e0f),
        //     U2 => Edges::from_magic(0x03010200040506070b090a080c0d0e0f),
        //     U3 => Edges::from_magic(0x4b0102480405060740090a430c0d0e0f),
        //     D => Edges::from_magic(0x004a4903040506070841420b0c0d0e0f),
        //     D2 => Edges::from_magic(0x0002010304050607080a090b0c0d0e0f),
        //     D3 => Edges::from_magic(0x00494a03040506070842410b0c0d0e0f),
        //     F => Edges::from_magic(0x171402031005061108090a0b0c0d0e0f),
        //     F2 => Edges::from_magic(0x010002030705060408090a0b0c0d0e0f),
        //     F3 => Edges::from_magic(0x141702031105061008090a0b0c0d0e0f),
        //     B => Edges::from_magic(0x000116150412130708090a0b0c0d0e0f),
        //     B2 => Edges::from_magic(0x000103020406050708090a0b0c0d0e0f),
        //     B3 => Edges::from_magic(0x000115160413120708090a0b0c0d0e0f),
        //     R => Edges::from_magic(0x000102032928060724250a0b0c0d0e0f),
        //     R2 => Edges::from_magic(0x000102030504060709080a0b0c0d0e0f),
        //     R3 => Edges::from_magic(0x000102032829060725240a0b0c0d0e0f),
        //     L => Edges::from_magic(0x0001020304052a2b080927260c0d0e0f),
        //     L2 => Edges::from_magic(0x000102030405070608090b0a0c0d0e0f),
        //     L3 => Edges::from_magic(0x0001020304052b2a080926270c0d0e0f),
        // };
        // self.compose(c)
        // todo!()
    }

    pub fn apply_alg(&self, a: &Alg) -> Self {
        a.iter().fold(*self, |acc, m| {
            acc.apply_move(*m)
        })
    }
}

// CO methods
impl Corners {
    /// For the given slot, set CO relative to `axis`.
    pub fn set_co(self, slot: Corner, co: CO, axis: Axis) -> Self {
        match axis {
            Axis::UD => self.set_coud(slot, co),
            Axis::FB => self.set_cofb(slot, co),
            Axis::LR => self.set_colr(slot, co),
        }
    }

    /// For the given slot, set CO relative to the UD axis.
    pub fn set_coud(self, slot: Corner, co: CO) -> Self {
        let ret = ops::lane_set_coud(self.0, slot as usize, co as u8);
        Self(ret)
    }

    /// For the given slot, set CO relative to the FB axis.
    pub fn set_cofb(self, slot: Corner, co: CO) -> Self {
        let ret = ops::lane_set_cofb(self.0, slot as usize, co as u8);
        Self(ret)
    }

    /// For the given slot, set CO relative to the LR axis.
    pub fn set_colr(self, slot: Corner, co: CO) -> Self {
        let ret = ops::lane_set_colr(self.0, slot as usize, co as u8);
        Self(ret)
    }

    /// Get CO relative to the `axis` at the given slot.
    pub fn co(self, slot: Corner, axis: Axis) -> CO {
        match axis {
            Axis::UD => self.coud(slot),
            Axis::FB => self.cofb(slot),
            Axis::LR => self.colr(slot),
        }
    }

    /// Get CO relative to the UD axis at the given slot.
    pub fn coud(self, slot: Corner) -> CO {
        let ret = ops::lane_coud(self.0, slot as usize);
        CO::from(ret)
    }

    /// Get CO relative to the FB axis at the given slot.
    pub fn cofb(self, slot: Corner) -> CO {
        let ret = ops::lane_cofb(self.0, slot as usize);
        CO::from(ret)
    }
    
    /// Get CO relative to the LR axis at the given slot.
    pub fn colr(self, slot: Corner) -> CO {
        let ret = ops::lane_colr(self.0, slot as usize);
        CO::from(ret)
    }

    pub fn check(self) -> Result<(), String> {
        todo!()
    }

    /// Checks that CP is bijective.
    pub fn check_cp(self) -> Result<(), String> {
        let mut loc = [None; 8];
        for slot in Corner::all() {
            let c = self.at(slot).piece();
            if let Some(prev_slot) = loc[c as usize] {
                return Err(format!("Corner {} appears twice: first at {}, and at {}", c, prev_slot, slot));
            }
            loc[c as usize] = Some(slot);
        }
        Ok(())
    }

    pub fn check_co(self) -> Result<(), String> {
        todo!()
    }

    pub fn is_legal(self) -> bool {
        todo!()
    }

    pub fn is_even_parity(self) -> bool {
        todo!()
    }

    pub fn is_odd_parity(self) -> bool {
        todo!()
    }

    pub fn arm(self) -> usize {
        todo!()
    }

    pub fn drm(self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fmt::Debug;

    #[test]
    fn test_set_co() {
        let c = Corners::new();
        let s = format!("{:?}", c);
        expect!(
            &s,
            "
            slot:   UBL UBR UFR UFL DFL DFR DBR DBL  a
            corner: UBL UBR UFR UFL DFL DFR DBR DBL 
            coud:   0   0   0   0   0   0   0   0 
        ",
        );
    }

    #[test]
    fn test_cofb_colr() {
        let c = Corners::new().apply_alg(&Alg::try_from("R U F R2 U F'").unwrap());
        let s = format!("{:?}", c);
        expect!(
            &s,
            "
            slot:   UBL UBR UFR UFL DFL DFR DBR DBL
            corner: DFL UFL UBL UFR UBR DBR DFR DBL
            coud:   1   0   2   1   0   2   0   0
        ",
        );
        let s = format!("{:?}", Corner::all().map(|slot| c.cofb(slot)));
        expect!(&s, "[CCW, Solved, CW, Solved, CCW, CCW, CCW, Solved]");
        let s = format!("{:?}", Corner::all().map(|slot| c.colr(slot)));
        expect!(&s, "[CCW, Solved, CW, CW, CW, Solved, CW, Solved]");
    }

    fn foo() {
        let c = Corners::new().apply_move(Move::B);
        println!("{:?}", c.at(Corner::UBL).cofb())
    }

    // fn dbg() {
    //     use crate::Move;
    //     use super::Corners;
    //     use crate::Alg;
    //     let alg = Alg::try_from("R F").unwrap();
    //     let c = Corners::default() + alg;
    //     c.dbg();
    //     (c + c.inverse()).dbg();
    // }
}

// A: EP EO
// A^-1: EO^-1 EP^-1
