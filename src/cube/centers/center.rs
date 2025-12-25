#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Center {
    U = 0,
    R = 1,
    F = 2,
    D = 3,
    L = 4,
    B = 5,
}

impl From<u8> for Center {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl Center {
    pub const fn all() -> [Center; 6] {
        [
            Center::from_u8(0),
            Center::from_u8(1),
            Center::from_u8(2),
            Center::from_u8(3),
            Center::from_u8(4),
            Center::from_u8(5),
        ]
    }

    pub const fn from_u8(x: u8) -> Center {
        debug_assert!(x < 6);
        unsafe { std::mem::transmute::<u8, Center>(x) }
    }
}
