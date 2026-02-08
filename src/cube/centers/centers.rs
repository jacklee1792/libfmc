use super::ops;
use std::ops::Add;
use std::simd::u8x8;

use crate::*;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Centers(pub u8x8);

impl Add<Self> for Centers {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl Default for Centers {
    fn default() -> Self {
        Self::new()
    }
}

impl Centers {
    pub const fn new() -> Self {
        Self(ops::CENTERS_IDENT)
    }

    pub fn compose(self, rhs: Self) -> Self {
        Self(ops::compose(self.0, rhs.0))
    }

    pub const fn magic(self) -> u64 {
        u64::from_le_bytes(self.0.to_array())
    }

    pub const fn from_magic(m: u64) -> Self {
        Self(u8x8::from_array(m.to_le_bytes()))
    }

    pub fn cycle<I>(cycle: I) -> Self
    where
        I: IntoIterator<Item = Center>,
    {
        let cycle = cycle.into_iter().collect::<Vec<_>>();
        let mut xp = ops::CENTERS_IDENT;
        let n = cycle.len();
        for i in 0..n {
            let src = cycle[i] as u8;
            let dst = cycle[(i + 1) % n] as usize;
            xp.as_mut_array()[dst] = src;
        }
        Self(xp)
    }

    pub fn slice_move(m: SliceMove) -> Self {
        use SliceMove::*;
        match m {
            M => Centers::from_magic(0x0706030402000105),
            M2 => Centers::from_magic(0x0706020400050103),
            M3 => Centers::from_magic(0x0706000405030102),
            E => Centers::from_magic(0x0706010503040200),
            E2 => Centers::from_magic(0x0706020103050400),
            E3 => Centers::from_magic(0x0706040203010500),
            S => Centers::from_magic(0x0706050301020004),
            S2 => Centers::from_magic(0x0706050100020403),
            S3 => Centers::from_magic(0x0706050004020301),
        }
    }

    pub fn apply_slice_move(self, m: SliceMove) -> Self {
        self.compose(Self::slice_move(m))
    }

    pub fn rotation<R>(r: R) -> Self
    where
        R: Into<Rotation>,
    {
        use Rotation::*;
        let r = r.into();
        match r {
            X => Self::slice_move(SliceMove::M3),
            X2 => Self::slice_move(SliceMove::M2),
            X3 => Self::slice_move(SliceMove::M),
            Y => Self::slice_move(SliceMove::E3),
            Y2 => Self::slice_move(SliceMove::E2),
            Y3 => Self::slice_move(SliceMove::E),
            Z => Self::slice_move(SliceMove::S),
            Z2 => Self::slice_move(SliceMove::S2),
            Z3 => Self::slice_move(SliceMove::S3),
        }
    }

    pub fn apply_rotation<R>(self, r: R) -> Self
    where
        R: Into<Rotation>,
    {
        self.compose(Self::rotation(r))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_rotation() {
        use Rotation::*;
        let a = Centers::rotation(X) + Centers::rotation(Y);
        let b = Centers::rotation(Y) + Centers::rotation(Z);
        assert_eq!(a, b);
        let a = Centers::rotation(X) + Centers::rotation(Y);
        let b = Centers::rotation(Y) + Centers::rotation(X);
        assert_ne!(a, b);
    }

    #[test]
    fn test_gen_slice_magics() {
        let mut out = String::new();
        for m in SliceMove::all() {
            let (s, n) = m.decompose();
            let cycle = match s {
                Slice::M => Centers::cycle([Center::U, Center::F, Center::D, Center::B]),
                Slice::E => Centers::cycle([Center::F, Center::R, Center::B, Center::L]),
                Slice::S => Centers::cycle([Center::U, Center::R, Center::D, Center::L]),
            };
            let mut c = Centers::new();
            for _ in 0..n {
                c = c.compose(cycle);
            }
            out += &format!("{m:?} => Centers::from_magic(0x{:016x}),\n", c.magic());
        }
        expect!(
            &out,
            "
            M => Centers::from_magic(0x0706030402000105),
            M2 => Centers::from_magic(0x0706020400050103),
            M3 => Centers::from_magic(0x0706000405030102),
            E => Centers::from_magic(0x0706010503040200),
            E2 => Centers::from_magic(0x0706020103050400),
            E3 => Centers::from_magic(0x0706040203010500),
            S => Centers::from_magic(0x0706050301020004),
            S2 => Centers::from_magic(0x0706050100020403),
            S3 => Centers::from_magic(0x0706050004020301),
        "
        )
    }
}
