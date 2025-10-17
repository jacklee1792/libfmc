use std::simd::u8x16;
use std::simd::mask8x16;
use std::simd::cmp::{SimdOrd, SimdPartialEq};

use crate::CORNERS_IDENT;

// Lane Layout:
// EOUD EOLR EOFB EP EP EP EP
//
// u8x16, lower 4 bits for ep, upper 3 bits for eofb/eorl/eoud

// upper lanes should always be 12, 13, 14, 15 !!

// Suppose a corner `c` is currently at a slot `s`. If `c` belongs to the same HTR tetrad as `s`,
// then CO on all axes is the same at `s`. Otherwise, CO on FB and LR can be deduced from CO on
// UD with following.

pub const EP_IDENT: u8x16 = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
pub const EDGES_IDENT: u8x16 = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

pub const EP_LANE_MASK: u8 = 0b00001111;
pub const EP_MASK: u8x16 = u8x16::splat(EP_LANE_MASK);

pub const EPSLICE_LANE_MASK: u8 = 0b00001100;
pub const EPSLICE_MASK: u8x16 = u8x16::splat(EPSLICE_LANE_MASK);
pub const EPSLICE_SHIFT: u8 = 2;

pub const EO_LANE_MASK: u8 = 0b01110000;
pub const EO_MASK: u8x16 = u8x16::splat(EO_LANE_MASK);
pub const EO_SHIFT: u8 = 4;

pub const EOFB_LANE_MASK: u8 = 0b00010000;
pub const EOFB_MASK: u8x16 = u8x16::splat(EOFB_LANE_MASK);
pub const EOFB_SHIFT: u8 = 4;

pub const EOLR_LANE_MASK: u8 = 0b00100000;
pub const EOLR_MASK: u8x16 = u8x16::splat(EOLR_LANE_MASK);
pub const EOLR_SHIFT: u8 = 5;

pub const EOUD_LANE_MASK: u8 = 0b01000000;
pub const EOUD_MASK: u8x16 = u8x16::splat(EOUD_LANE_MASK);
pub const EOUD_SHIFT: u8 = 6;

/// Construct a edge vector from EO and EP vectors.
pub fn cons(eofb: u8x16, eolr: u8x16, eoud: u8x16, ep: u8x16) -> u8x16 {
    ep | (eofb << EOFB_SHIFT) | (eolr << EOLR_SHIFT) | (eoud << EOUD_SHIFT)
}

/// Construct a lane from EO and EP lane values.
pub fn lane_cons(eofb: u8, eolr: u8, eoud: u8, ep: u8) -> u8 {
    ep | (eofb << EOFB_SHIFT) | (eolr << EOLR_SHIFT) | (eoud << EOUD_SHIFT)
}

/// Mask out the EP bits.
pub fn ep(a: u8x16) -> u8x16 {
    a & EP_MASK
}

/// Mask out the EP bits at a slot.
pub fn lane_ep(a: u8x16, i: usize) -> u8 {
    a[i] & EP_LANE_MASK
}

/// Get a vector corresponding to EO on FB.
pub fn eofb(a: u8x16) -> u8x16 {
    (a & EOFB_MASK) >> EOFB_SHIFT
} 

/// Get a vector corresponding to EO on LR.
pub fn eolr(a: u8x16) -> u8x16 {
    (a & EOLR_MASK) >> EOLR_SHIFT
} 

/// Get a vector corresponding to EO on UD.
pub fn eoud(a: u8x16) -> u8x16 {
    (a & EOUD_MASK) >> EOUD_SHIFT
}

/// Get EO on FB at a slot.
pub fn lane_eofb(a: u8x16, i: usize) -> u8 {
    (a[i] & EOFB_LANE_MASK) >> EOFB_SHIFT
}

/// Get EO on LR at a slot.
pub fn lane_eolr(a: u8x16, i: usize) -> u8 {
    (a[i] & EOLR_LANE_MASK) >> EOLR_SHIFT
}

/// Get EO on UD at a slot.
pub fn lane_eoud(a: u8x16, i: usize) -> u8 {
    (a[i] & EOUD_LANE_MASK) >> EOUD_SHIFT
}

/// Select the edges which are not in HTR, using the fact that bits 2 and 3 encodes which
/// slice an edge belongs to (see `Edge` definition)
pub fn htrbad(a: u8x16) -> mask8x16 {
    let diff = (a ^ EDGES_IDENT) & u8x16::splat(0b1100);
    diff.simd_ne(u8x16::splat(0))
}

/// Whether the corner in the given slot is in HTR.
pub fn lane_htrbad(a: u8x16, i: usize) -> bool {
    (a[i] ^ (i as u8)) & 0b1100 != 0
}

/// Return a new vector with EP set to the given vector.
/// DANGER ! ep should have upper lanes as the identity
pub fn set_ep(a: u8x16, ep: u8x16) -> u8x16 {
    (a & EO_MASK) ^ ep
}

/// Return a new vector with EP at the given slot set to a new value.
pub fn lane_set_ep(mut a: u8x16, i: usize, ep: u8) -> u8x16 {
    a[i] = (a[i] & EO_LANE_MASK) ^ ep;
    a
}

// S/S, M/M, E/E
// 	- xor=0
// 	- fb <> ud: 0
// 	- fb <> lr: 0
// S/M
// 	- xor=1
// 	- fb <> ud: 1
// 	- fb <> lr: 0
// S/E
// 	- xor=2
// 	- fb <> ud: 0
// 	- fb <> lr: 1
// M/E ->
// 	- xor=3
// 	- fb <> ud: 1
// 	- fb <> lr: 1


// one-hot encoding, eofb to eoud
pub fn eofb_to_eoud(ep: u8x16) -> u8x16 {
    let s = ep ^ EDGES_IDENT;
    (u8x16::splat(0b1010) >> s) & u8x16::splat(1)
}

// one-hot encoding, eofb to eolr
pub fn eofb_to_eolr(ep: u8x16) -> u8x16 {
    let s = ep ^ EDGES_IDENT;
    (u8x16::splat(0b1100) >> s) & u8x16::splat(1)
}

// one hot-encoding, eolr to eoud
pub fn eolr_to_eoud(ep: u8x16) -> u8x16 {
    let s = ep ^ EDGES_IDENT;
    (u8x16::splat(0b0110) >> s) & u8x16::splat(1)
}

/// Return a new vector with EOFB set to the given vector.
pub fn set_eofb(a: u8x16, eofb: u8x16) -> u8x16 {
    let ep = a & EP_MASK;
    let eolr = eofb ^ eofb_to_eolr(ep);
    let eoud = eofb ^ eofb_to_eoud(ep);
    cons(eofb, eolr, eoud, ep)
}

/// Return a new vector with EOLR set to the given vector.
pub fn set_eolr(a: u8x16, eolr: u8x16) -> u8x16 {
    let ep = a & EP_MASK;
    let eofb = eolr ^ eofb_to_eolr(ep);
    let eoud = eolr ^ eolr_to_eoud(ep);
    cons(eofb, eolr, eoud, ep)
}

/// Return a new vector with EOUD set to the given vector.
pub fn set_eoud(a: u8x16, eoud: u8x16) -> u8x16 {
    let ep = a & EP_MASK;
    let eofb = eoud ^ eofb_to_eoud(ep);
    let eolr = eoud ^ eolr_to_eoud(ep);
    cons(eofb, eolr, eoud, ep)
}

