use crate::cube::edges::ops;
use crate::*;
use std::fmt;
use std::fmt::Debug;
use std::ops::Add;
use std::simd::cmp::SimdPartialEq;
use std::simd::u8x16;
use std::simd::mask8x16;

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
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Edges(pub(super) u8x16);

impl Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_eo = |eo: EO| match eo {
            EO::Solved => ".",
            EO::Flipped => "X",
        };
        let index = Edge::all().map(|e| format!("{:>2}", e)).join(" ");
        let ep = Edge::all().map(|e| format!("{:>2}", self.at(e))).join(" ");
        let eofb = Edge::all()
            .map(|e| format!("{:>2}", fmt_eo(self.eofb(e))))
            .join(" ");
        let eolr = Edge::all()
            .map(|e| format!("{:>2}", fmt_eo(self.eolr(e))))
            .join(" ");
        let eoud = Edge::all()
            .map(|e| format!("{:>2}", fmt_eo(self.eoud(e))))
            .join(" ");
        writeln!(f, " slot: {index}")?;
        writeln!(f, "piece: {ep}")?;
        writeln!(f, " eofb: {eofb}")?;
        writeln!(f, " eolr: {eolr}")?;
        writeln!(f, " eoud: {eoud}")?;
        Ok(())
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
        Self(u8x16::from_array([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ]))
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

    pub const fn magic(self) -> u128 {
        u128::from_le_bytes(self.0.to_array())
    }

    /// Modify EP by specifying for each location which corner populates it. Preserves EO on FB.
    pub fn set_ep_passive<F>(self, mut f: F) -> Self
    where
        F: FnMut(Edge) -> Edge,
    {
        let mut a = self.0;
        for slot in (0..12).map(Edge::from) {
            let piece = f(slot);
            a = ops::lane_set_ep(a, slot as usize, piece as u8);
        }
        Self(a)
    }

    /// Modify CP by specifying for each corner where it's located. Preserves CO on UD.
    pub fn set_cp_active<F>(&self, mut f: F) -> Self
    where
        F: FnMut(Edge) -> Edge,
    {
        let mut a = self.0;
        for piece in (0..12).map(Edge::from) {
            let slot = f(piece) as usize;
            a = ops::lane_set_ep(a, slot as usize, piece as u8);
        }
        Self(a)
    }

    pub fn cycle<I>(cycle: I, axis: Axis) -> Self
    where
        I: IntoIterator<Item = Edge>,
    {
        match axis {
            Axis::UD => Self::cycle_eoud(cycle),
            Axis::FB => Self::cycle_eofb(cycle),
            Axis::LR => Self::cycle_eolr(cycle),
        }
    }

    /// Just cycle, preserving all EO. Will likely leave the cube in an invalid state
    fn cycle_ep_only<I>(cycle: I) -> u8x16
    where
        I: IntoIterator<Item = Edge>,
    {
        let cycle = cycle.into_iter().collect::<Vec<_>>();
        let mut ep = ops::EDGES_IDENT;
        let n = cycle.len();
        for i in 0..n {
            let src = cycle[i] as u8;
            let dst = cycle[(i + 1) % n] as usize;
            ep[dst] = src;
        }
        ep
    }

    /// A cycle which preserves EO on FB. The first slot in the cycle is sent to the second
    /// slot in the cycle, and so on with the last slot in the cycle being sent to the first slot
    /// in the cycle.
    pub fn cycle_eofb<I>(cycle: I) -> Self
    where
        I: IntoIterator<Item = Edge>,
    {
        let ep = Self::cycle_ep_only(cycle);
        Self(ops::set_eofb(ep, ops::eofb(ep)))
    }

    /// A cycle which preserves EO on LR. The first slot in the cycle is sent to the second
    /// slot in the cycle, and so on with the last slot in the cycle being sent to the first slot
    /// in the cycle.
    pub fn cycle_eolr<I>(cycle: I) -> Self
    where
        I: IntoIterator<Item = Edge>,
    {
        let ep = Self::cycle_ep_only(cycle);
        Self(ops::set_eolr(ep, ops::eolr(ep)))
    }

    /// A cycle which preserves EO on UD. The first slot in the cycle is sent to the second
    /// slot in the cycle, and so on with the last slot in the cycle being sent to the first slot
    /// in the cycle.
    pub fn cycle_eoud<I>(cycle: I) -> Self
    where
        I: IntoIterator<Item = Edge>,
    {
        let ep = Self::cycle_ep_only(cycle);
        Self(ops::set_eoud(ep, ops::eoud(ep)))
    }

    /// Flip the given edges. If an edge is provided multiple times, it will flip once for each
    /// time it's provided.
    pub fn flip<I>(edges: I) -> Self
    where I: IntoIterator<Item = Edge>
    {
        let mut flips = [false; 16];
        for edge in edges.into_iter() {
            flips[edge as usize] = !flips[edge as usize]
        }
        Self(ops::flip(mask8x16::from_array(flips)))
    }

    pub fn compose(self, rhs: Self) -> Self {
        Self(ops::compose(self.0, rhs.0))
    }

    /// The edge which current resides at the given slot.
    pub fn at(self, slot: Edge) -> Edge {
        Edge::from(ops::lane_ep(self.0, slot as usize))
    }

    // EO relative to FB axis.
    pub fn eofb(self, slot: Edge) -> EO {
        ops::lane_eofb(self.0, slot as usize).into()
    }

    // EO relative to LR axis.
    pub fn eolr(self, slot: Edge) -> EO {
        ops::lane_eolr(self.0, slot as usize).into()
    }

    // EO relative to UD axis.
    pub fn eoud(self, slot: Edge) -> EO {
        ops::lane_eoud(self.0, slot as usize).into()
    }

    pub fn inverse(self) -> Self {
        // ok normally you have A.ep + A.eo
        // inverse is ident + A.eo^-1 + A.ep^-1
        let inv_eo = (self.0 & ops::EO_MASK) ^ ops::EO_MASK;
        let inv_ep = ops::inv_ep(ops::ep(self.0));
        let inv = ops::compose_ep(ops::EDGES_IDENT | inv_eo, inv_ep);
        Self(inv)
    }

    pub fn mov(m: Move) -> Self {
        use Move::*;
        match m {
            U => Edges::from_magic(0xf0e0d0c0b0a09084306054044020147),
            U2 => Edges::from_magic(0xf0e0d0c0b0a09080406050700020103),
            U3 => Edges::from_magic(0xf0e0d0c0b0a09084006054347020144),
            D => Edges::from_magic(0xf0e0d0c0b0a09080741420403464500),
            D2 => Edges::from_magic(0xf0e0d0c0b0a09080705060403010200),
            D3 => Edges::from_magic(0xf0e0d0c0b0a09080742410403454600),
            F => Edges::from_magic(0xf0e0d0c150a09140706181b03020100),
            F2 => Edges::from_magic(0xf0e0d0c080a090b0706040503020100),
            F3 => Edges::from_magic(0xf0e0d0c140a091507061b1803020100),
            B => Edges::from_magic(0xf0e0d0c0b171608191a050403020100),
            B2 => Edges::from_magic(0xf0e0d0c0b090a080607050403020100),
            B3 => Edges::from_magic(0xf0e0d0c0b1617081a19050403020100),
            R => Edges::from_magic(0xf0e0d0c0b0a20210706050403022928),
            R2 => Edges::from_magic(0xf0e0d0c0b0a08090706050403020001),
            R3 => Edges::from_magic(0xf0e0d0c0b0a21200706050403022829),
            L => Edges::from_magic(0xf0e0d0c23220908070605042a2b0100),
            L2 => Edges::from_magic(0xf0e0d0c0a0b09080706050402030100),
            L3 => Edges::from_magic(0xf0e0d0c22230908070605042b2a0100),
        }
    }

    pub fn apply_move(self, m: Move) -> Self {
        self.compose(Self::mov(m))
    }

    pub fn apply_alg(self, a: &Alg) -> Self {
        a.iter().fold(self, |acc, m| acc.apply_move(m))
    }

    pub fn apply_sym(self, s: Sym) -> Self {
        self + s.cube().edges
    }

    pub fn conjugate_sym(self, s: Sym) -> Self {
        Edges::new().apply_sym(s).compose(self).apply_sym(s.inverse())
    }

    pub fn index_eofb(self) -> usize {
        ops::index_eofb(self.0)
    }

    pub fn unindex_eofb(coord: usize) -> Self {
        Edges(ops::unindex_eofb(coord))
    }

    pub fn is_drud(self) -> bool {
        self.is_eofb() && self.is_eolr()
    }

    pub fn is_eofb(self) -> bool {
        ops::eofb(self.0) == u8x16::splat(0)
    }
    
    pub fn is_eolr(self) -> bool {
        ops::eolr(self.0) == u8x16::splat(0)
    }
    
    pub fn is_eoud(self) -> bool {
        ops::eoud(self.0) == u8x16::splat(0)
    }

    pub fn is_solved(self) -> bool {
        self.0 == ops::EDGES_IDENT
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_set_eo() {
        let alg =
            Alg::try_from("R' U' F R' B2 R L' B R L F B' L2 U' F2 U' D2 F2 U' R2 L2 B R' U' F")
                .unwrap();
        let e = Edges::new().apply_alg(&alg);
        let s = format!("{:?}", e);
        expect!(
            &s,
            "
            slot: UR DR DL UL UF DF DB UB FR BR BL FL
           piece: UL DR DF FL UR DB UB DL BR UF BL FR
            eofb:  X  .  .  X  X  .  X  X  .  X  .  .
            eolr:  X  .  .  .  X  .  X  X  .  .  .  .
            eoud:  X  .  X  X  .  .  X  .  .  .  .  .
        "
        );
    }
}
