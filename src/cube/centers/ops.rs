use std::simd::u8x8;

pub const CENTERS_IDENT: u8x8 = u8x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]);

pub fn compose(a: u8x8, b: u8x8) -> u8x8 {
    a.swizzle_dyn(b)
}
