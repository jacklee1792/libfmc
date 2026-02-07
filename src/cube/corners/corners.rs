use crate::*;
use std::{
    ops::Add,
    simd::{
        cmp::{SimdOrd, SimdPartialEq},
        u8x8,
    },
    fmt::{self, Debug},
};
use super::ops;

/// Corners of the cube.
///
/// ### Representation
/// Corner ordering:
///   UBL UFR DFL DBR UBR UFL DFR DBL
///
/// The underlying representation is a SIMD vector of 8 lanes, each lane representing a corner slot.
/// Each lane has a packed 8-bit representation, where:
/// - Bits 0..=2 represent the corner piece at that position (0..=7).
/// - Bits 3..=4 represent the orientation of the corner piece with respect to the UD-axis.
///   - 0 means the corner is solved;
///   - 1 means the corner is twisted counterclockwise;
///   - 2 means the corner is twisted clockwise.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Corners(pub u8x8);

impl Debug for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:8}", "slot:")?;
        for slot in Corner::all() {
            write!(f, "{:<4}", format!("{:?}", slot))?;
        }
        writeln!(f)?;

        let cp = ops::cp(self.0)
            .as_array()
            .iter()
            .map(|e| Corner::from(*e))
            .collect::<Vec<_>>();

        write!(f, "{:8}", "corner:")?;
        for corner in &cp {
            write!(f, "{:<4}", format!("{:?}", corner))?;
        }
        writeln!(f)?;

        let coud = ops::coud(self.0)
            .as_array()
            .iter()
            .map(|&x| x as usize)
            .collect::<Vec<_>>();
        write!(f, "{:8}", "coud:")?;
        for v in &coud {
            write!(f, "{:<4}", v)?;
        }
        writeln!(f)
    }
}

impl Default for Corners {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<Self> for Corners {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl Add<Move> for Corners {
    type Output = Self;

    fn add(self, rhs: Move) -> Self::Output {
        self.apply_move(rhs)
    }
}

impl Add<Alg> for Corners {
    type Output = Self;

    fn add(self, rhs: Alg) -> Self::Output {
        self.apply_alg(&rhs)
    }
}

impl Corners {
    /// Construct a new `Corners` instance with the identity CP and CO.
    pub const fn new() -> Self {
        Self(u8x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]))
    }

    /// A view of the piece at the given slot.
    pub fn at(&self, slot: Corner) -> CornerRef<'_> {
        CornerRef { slot, corners: self }
    }

    /// The slot where the given `Corner` currently resides.
    pub fn find(self, corner: Corner) -> Corner {
        let target = u8x8::splat(corner as u8);
        let cp = self.0 & CP_MASK;
        let slot = cp.simd_eq(target).first_set().unwrap() as u8;
        Corner::from(slot)
    }

    pub const fn from_magic(m: u64) -> Self {
        Self(u8x8::from_array(m.to_le_bytes()))
    }

    pub const fn magic(self) -> u64 {
        u64::from_le_bytes(self.0.to_array())
    }
    
    /// Modify CP by specifying for each location which corner populates it. Preserves CO on UD.
    pub fn set_cp_passive<F>(&self, mut f: F) -> Self
    where
        F: FnMut(Corner) -> Corner,
    {
        let mut a = self.0.to_array();
        for (i, lane) in a.iter_mut().enumerate() {
            let slot = Corner::from(i as u8);
            let piece = f(slot);
            *lane = (*lane & CO_LANE_MASK) | (piece as u8);
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
        use Move::*;
        let c = match m {
            U => Corners::from_magic(0x0706050402010003),
            U2 => Corners::from_magic(0x0706050401000302),
            U3 => Corners::from_magic(0x0706050400030201),
            D => Corners::from_magic(0x0605040703020100),
            D2 => Corners::from_magic(0x0504070603020100),
            D3 => Corners::from_magic(0x0407060503020100),
            F => Corners::from_magic(0x07060a150c130100),
            F2 => Corners::from_magic(0x0706030205040100),
            F3 => Corners::from_magic(0x07060c130a150100),
            B => Corners::from_magic(0x0817050403020e11),
            B2 => Corners::from_magic(0x0100050403020706),
            B3 => Corners::from_magic(0x0e11050403020817),
            R => Corners::from_magic(0x07091604030d1200),
            R2 => Corners::from_magic(0x0702010403060500),
            R3 => Corners::from_magic(0x070d120403091600),
            L => Corners::from_magic(0x1406050b1002010f),
            L2 => Corners::from_magic(0x0306050007020104),
            L3 => Corners::from_magic(0x1006050f1402010b),
        };
        self.compose(c)
    }

    pub fn apply_rotation(self, r: Rotation) -> Self {
        todo!()
    }

    pub fn apply_alg(&self, a: &Alg) -> Self {
        a.iter().fold(*self, |acc, m| {
            acc.apply_move(m)
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

    pub fn is_drud(self) -> bool {
        ops::coud(self.0) == u8x8::splat(0)
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

    pub fn perm_coord(self) -> usize {
        ops::prank(ops::cp(self.0))
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
            slot:   UBL UBR UFR UFL DFL DFR DBR DBL
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

    #[test]
    fn test_gen_apply_move() {
        let mut s = String::new();
        for m in Move::all() {
            let (f, n) = m.decompose();
            use Face::*;
            use Corner::*;
            let cycle = match f {
                U => Corners::cycle_drud([UFR, UFL, UBL, UBR]),
                F => Corners::cycle_drfb([UFR, DFR, DFL, UFL]),
                R => Corners::cycle_drlr([UFR, UBR, DBR, DFR]),
                D => Corners::cycle_drud([DFL, DFR, DBR, DBL]),
                B => Corners::cycle_drfb([UBR, UBL, DBL, DBR]),
                L => Corners::cycle_drlr([UFL, DFL, DBL, UBL]),
            };
            let mut c = Corners::new();
            for _ in 0..n {
                c = c.compose(cycle);
            }
            s += &format!("{m:?} => Corners::from_magic(0x{:016x}),\n", c.magic());
        }
        expect!(
            &s,
            "
            U => Corners::from_magic(0x0706050402010003),
            U2 => Corners::from_magic(0x0706050401000302),
            U3 => Corners::from_magic(0x0706050400030201),
            D => Corners::from_magic(0x0605040703020100),
            D2 => Corners::from_magic(0x0504070603020100),
            D3 => Corners::from_magic(0x0407060503020100),
            F => Corners::from_magic(0x07060a150c130100),
            F2 => Corners::from_magic(0x0706030205040100),
            F3 => Corners::from_magic(0x07060c130a150100),
            B => Corners::from_magic(0x0817050403020e11),
            B2 => Corners::from_magic(0x0100050403020706),
            B3 => Corners::from_magic(0x0e11050403020817),
            R => Corners::from_magic(0x07091604030d1200),
            R2 => Corners::from_magic(0x0702010403060500),
            R3 => Corners::from_magic(0x070d120403091600),
            L => Corners::from_magic(0x1406050b1002010f),
            L2 => Corners::from_magic(0x0306050007020104),
            L3 => Corners::from_magic(0x1006050f1402010b),
        ")
    }

    fn test_gen_rotate() {
        let mut s = String::new();
        for m in Move::all() {
            let (f, n) = m.decompose();
            use Face::*;
            use Corner::*;
            let cycle = match f {
                U => Corners::cycle_drud([UFR, UFL, UBL, UBR]),
                F => Corners::cycle_drfb([UFR, DFR, DFL, UFL]),
                R => Corners::cycle_drlr([UFR, UBR, DBR, DFR]),
                D => Corners::cycle_drud([DFL, DFR, DBR, DBL]),
                B => Corners::cycle_drfb([UBR, UBL, DBL, DBR]),
                L => Corners::cycle_drlr([UFL, DFL, DBL, UBL]),
            };
            let mut c = Corners::new();
            for _ in 0..n {
                c = c.compose(cycle);
            }
            s += &format!("{m:?} => Corners::from_magic(0x{:016x}),\n", c.magic());
        }
        expect!(
            &s,
            "
            U => Corners::from_magic(0x0706050402010003),
            U2 => Corners::from_magic(0x0706050401000302),
            U3 => Corners::from_magic(0x0706050400030201),
            D => Corners::from_magic(0x0605040703020100),
            D2 => Corners::from_magic(0x0504070603020100),
            D3 => Corners::from_magic(0x0407060503020100),
            F => Corners::from_magic(0x07060a150c130100),
            F2 => Corners::from_magic(0x0706030205040100),
            F3 => Corners::from_magic(0x07060c130a150100),
            B => Corners::from_magic(0x0817050403020e11),
            B2 => Corners::from_magic(0x0100050403020706),
            B3 => Corners::from_magic(0x0e11050403020817),
            R => Corners::from_magic(0x07091604030d1200),
            R2 => Corners::from_magic(0x0702010403060500),
            R3 => Corners::from_magic(0x070d120403091600),
            L => Corners::from_magic(0x1406050b1002010f),
            L2 => Corners::from_magic(0x0306050007020104),
            L3 => Corners::from_magic(0x1006050f1402010b),
        ")
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
