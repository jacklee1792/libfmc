use std::simd::u8x8;
use std::simd::mask8x8;
use std::simd::cmp::{SimdOrd, SimdPartialEq};

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
    if lane_htrbad(a, i) {
        cofb
    } else {
        coud
    }
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
    if lane_htrbad(a, i) {
        colr
    } else {
        coud
    }
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
    let coud = if lane_htrbad(a, i) {
        coud
    } else {
        cofb
    };
    lane_set_coud(a, i, coud)
}

pub fn set_colr(a: u8x8, colr: u8x8) -> u8x8 {
    let coud = addmod3(colr, COLR_TO_COUD);
    let coud = htrbad(a).select(coud, colr);
    set_coud(a, coud)
}

pub fn lane_set_colr(a: u8x8, i: usize, colr: u8) -> u8x8 {
    let coud = (colr + COLR_TO_COUD[i]) % 3;
    let coud = if lane_htrbad(a, i) {
        coud
    } else {
        colr
    };
    lane_set_coud(a, i, coud)
}
