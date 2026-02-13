use std::ops::{Div, Rem};
use std::simd::cmp::{SimdOrd, SimdPartialEq};
use std::simd::mask8x8;
use std::simd::num::SimdUint;
use std::simd::u8x8;
use std::simd::usizex8;

// Suppose a corner `c` is currently at a slot `s`. If `c` belongs to the same HTR tetrad as `s`,
// then CO on all axes is the same at `s`. Otherwise, CO on FB and LR can be deduced from CO on
// UD with following.

pub const COUD_TO_COFB: u8x8 = u8x8::from_array([1, 2, 1, 2, 1, 2, 1, 2]);
pub const COUD_TO_COLR: u8x8 = u8x8::from_array([2, 1, 2, 1, 2, 1, 2, 1]);
pub const COFB_TO_COUD: u8x8 = COUD_TO_COLR;
pub const COLR_TO_COUD: u8x8 = COUD_TO_COFB;

pub const CP_IDENT: u8x8 = u8x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]);
pub const CORNERS_IDENT: u8x8 = u8x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]);

pub const CP_LANE_MASK: u8 = 0b00000111;
pub const CP_MASK: u8x8 = u8x8::splat(CP_LANE_MASK);

pub const CO_LANE_MASK: u8 = 0b00011000;
pub const CO_MASK: u8x8 = u8x8::splat(CO_LANE_MASK);
pub const CO_SHIFT: u8 = 3;

/// Construct a corner vector from CO and CP vectors.
pub fn cons(coud: u8x8, cp: u8x8) -> u8x8 {
    (coud << CO_SHIFT) + cp
}

/// Construct a lane from CO and CP lane values.
pub fn lane_cons(coud: u8, cp: u8) -> u8 {
    (coud << CO_SHIFT) + cp
}

/// Mask out the CP bits.
pub fn cp(a: u8x8) -> u8x8 {
    a & CP_MASK
}

/// Mask out the CP bits at a slot.
pub fn lane_cp(a: u8x8, i: usize) -> u8 {
    a[i] & CP_LANE_MASK
}

/// Get a vector corresponding to CO on UD.
pub fn coud(a: u8x8) -> u8x8 {
    (a & CO_MASK) >> CO_SHIFT
}

/// Get CO on UD at a slot.
pub fn lane_coud(a: u8x8, i: usize) -> u8 {
    (a[i] & CO_LANE_MASK) >> CO_SHIFT
}

/// Get a vector corresponding to CO on FB.
pub fn cofb(a: u8x8) -> u8x8 {
    let coud = coud(a);
    let cofb = addmod3(coud, COUD_TO_COFB);
    htrbad(a).select(cofb, coud)
}

/// Get CO on FB at a slot.
pub fn lane_cofb(a: u8x8, i: usize) -> u8 {
    let coud = lane_coud(a, i);
    let cofb = (coud + COUD_TO_COFB[i]) % 3;
    if lane_htrbad(a, i) { cofb } else { coud }
}

/// Get a vector corresponding to CO on LR.
pub fn colr(a: u8x8) -> u8x8 {
    let coud = coud(a);
    let colr = addmod3(coud, COUD_TO_COLR);
    let bad = htrbad(a);
    bad.select(colr, coud)
}

/// Get CO on LR at a slot.
pub fn lane_colr(a: u8x8, i: usize) -> u8 {
    let coud = lane_coud(a, i);
    let colr = (coud + COUD_TO_COLR[i]) % 3;
    if lane_htrbad(a, i) { colr } else { coud }
}

/// Modular additive inverse of a vector mod 3.
pub fn invmod3(a: u8x8) -> u8x8 {
    let three = u8x8::splat(3);
    let ret = three - a;
    ret.simd_ne(three).select(ret, u8x8::splat(0))
}

/// Add two vectors mod 3, assuming the vectors already contain elements mod 3.
pub fn addmod3(a: u8x8, b: u8x8) -> u8x8 {
    let s = a + b;
    s.simd_min(s - u8x8::splat(3))
}

/// Select the corners which are not in HTR, using the fact that the LSB encodes which
/// tetrad a corner belongs to.
pub fn htrbad(a: u8x8) -> mask8x8 {
    ((a ^ CORNERS_IDENT) & u8x8::splat(1)).simd_ne(u8x8::splat(0))
}

/// Whether the corner in the given slot is in HTR.
pub fn lane_htrbad(a: u8x8, i: usize) -> bool {
    (a[i] ^ (i as u8)) & 1 != 0
}

/// Return a new vector with CP set to the given vector.
pub fn set_cp(a: u8x8, cp: u8x8) -> u8x8 {
    (a & CO_MASK) ^ cp
}

/// Return a new vector with CP at the given slot set to a new value.
pub fn lane_set_cp(mut a: u8x8, i: usize, cp: u8) -> u8x8 {
    a[i] = (a[i] & CO_LANE_MASK) ^ cp;
    a
}

/// Return a new vector with COUD set to the given vector.
pub fn set_coud(a: u8x8, coud: u8x8) -> u8x8 {
    (coud << CO_SHIFT) ^ (a & CP_MASK)
}

pub fn lane_set_coud(mut a: u8x8, i: usize, coud: u8) -> u8x8 {
    a[i] = (coud << CO_SHIFT) ^ (a[i] & CP_LANE_MASK);
    a
}

pub fn set_cofb(a: u8x8, cofb: u8x8) -> u8x8 {
    let coud = addmod3(cofb, COFB_TO_COUD);
    let coud = htrbad(a).select(coud, cofb);
    set_coud(a, coud)
}

pub fn lane_set_cofb(a: u8x8, i: usize, cofb: u8) -> u8x8 {
    let coud = (cofb + COFB_TO_COUD[i]) % 3;
    let coud = if lane_htrbad(a, i) { coud } else { cofb };
    lane_set_coud(a, i, coud)
}

pub fn set_colr(a: u8x8, colr: u8x8) -> u8x8 {
    let coud = addmod3(colr, COLR_TO_COUD);
    let coud = htrbad(a).select(coud, colr);
    set_coud(a, coud)
}

pub fn lane_set_colr(a: u8x8, i: usize, colr: u8) -> u8x8 {
    let coud = (colr + COLR_TO_COUD[i]) % 3;
    let coud = if lane_htrbad(a, i) { coud } else { colr };
    lane_set_coud(a, i, coud)
}

/// select corners which have ud sticker facing fb
pub fn armlr(_a: u8x8) -> mask8x8 {
    todo!()
}

/// Given a CO vector, keep 0s and swap 1s with 2s (and vice versa).
pub fn invert_nonzero(co: u8x8) -> u8x8 {
    let zero = u8x8::splat(0);
    let three = u8x8::splat(3);
    co.simd_eq(zero).select(zero, three - co)
}

/// Given a vector A, return a new vector B where B[i] encodes the number of
/// inversions in A that have left endpoint at i, i.e. the count of A[i] > A[j] for i < j
pub fn lehmer(a: u8x8) -> u8x8 {
    let mut unused = 0xffff;
    let mut ret = u8x8::splat(0);
    for i in 0..8 {
        let j = 1u16.wrapping_shl(a[i] as u32);
        ret[i] = (unused & (j - 1)).count_ones() as u8;
        unused ^= j;
    }
    ret
}

/// Inverse operation of `lehmer`. Behaviour is undefined if the inversion counts
/// do not correspond to a real permutation.
pub fn unlehmer(a: u8x8) -> u8x8 {
    let mut unused: usize = 0x876543210;
    let mut ret = u8x8::splat(0);
    for i in 0..8 {
        let invs = a[i];
        let above = unused >> (4 * (invs + 1));
        let below = unused & (1 << (4 * invs)) - 1;
        ret[i] = ((unused >> (4 * invs)) & 0xf) as u8;
        unused = above << (4 * invs) | below;
    }
    ret
}

/// Rank of a permutation. (it's just a prank bro!!!)
pub fn prank(a: u8x8) -> usize {
    let fact = usizex8::from_array([5040, 720, 120, 24, 6, 2, 1, 1]);
    (fact * lehmer(a).cast()).reduce_sum()
}

/// Inverse of `prank`. Behaviour is undefined if the rank is too big to correspond
/// to a permutation.
pub fn unprank(mut rank: usize) -> u8x8 {
    let fact = usizex8::from_array([5040, 720, 120, 24, 6, 2, 1, 1]);
    let mut l = u8x8::splat(0);
    for i in 0..8 {
        l[i] = (rank / fact[i]) as u8;
        rank %= fact[i];
    }
    unlehmer(l)
}

pub fn index_coud(a: u8x8) -> usize {
    const MULT: usizex8 = usizex8::from_array([1, 3, 9, 27, 81, 243, 729, 0]);
    (coud(a).cast() * MULT).reduce_sum()
}

pub fn unindex_coud(coord: usize) -> u8x8 {
    debug_assert!(coord < 2187);
    const MULT: usizex8 = usizex8::from_array([1, 3, 9, 27, 81, 243, 729, 2187]);
    const THREE: usizex8 = usizex8::splat(3);
    let mut coud = usizex8::splat(coord).div(MULT).rem(THREE);
    // infer CO of last corner
    coud[7] = match coud.reduce_sum() % 3 {
        0 => 0,
        1 => 2,
        2 => 1,
        _ => unreachable!(),
    };
    set_coud(CORNERS_IDENT, coud.cast())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_prank_roundtrip() {
        for rank in 0..40320 {
            let p = unprank(rank);
            assert_eq!(rank, prank(p));
        }
    }

    #[test]
    fn test_coud_roundtrip() {
        let mut cases = HashSet::<u8x8>::new();
        for coord in 0..2187 {
            let case = unindex_coud(coord);
            assert_eq!(coord, index_coud(case));
            cases.insert(case);
        }
        assert_eq!(cases.len(), 2187);
    }
}
