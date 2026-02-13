use crate::*;
use std::array;
use std::ops::Add;
use std::sync::LazyLock;

static SYM_CUBE: LazyLock<[Cube; 48]> = LazyLock::new(|| {
    let m_edges = [Edge::UF, Edge::UB, Edge::DB, Edge::DF];
    let e_edges = [Edge::FR, Edge::FL, Edge::BL, Edge::BR];
    let s_edges = [Edge::UR, Edge::DR, Edge::DL, Edge::UL];
    let m3 = Cube::from(Edges::cycle_eolr(m_edges) + Edges::flip(m_edges));
    let e3 = Cube::from(Edges::cycle_eoud(e_edges) + Edges::flip(e_edges));
    let s = Cube::from(Edges::cycle_eolr(s_edges) + Edges::flip(s_edges));

    let lr = Cube {
        edges: (Edges::cycle_eofb([Edge::UL, Edge::UR])
            + Edges::cycle_eofb([Edge::DL, Edge::DR])
            + Edges::cycle_eofb([Edge::FL, Edge::FR])
            + Edges::cycle_eofb([Edge::BL, Edge::BR])),
        corners: (Corners::cycle_drud([Corner::UFL, Corner::UFR])
            + Corners::cycle_drud([Corner::UBL, Corner::UBR])
            + Corners::cycle_drud([Corner::DFL, Corner::DFR])
            + Corners::cycle_drud([Corner::DBL, Corner::DBR])),
    };
    let mut ret: [Option<Cube>; 48] = [None; 48];

    // TODO make less ugly
    use Move::*;
    use Sym::*;
    ret[UF as usize] = Some(Cube::default());
    ret[FD as usize] = Some(Cube::from(alg![R, L3]) + m3); // x

    ret[DB as usize] = Some(ret[FD as usize].unwrap() + ret[FD as usize].unwrap()); // x2
    ret[BU as usize] = Some(ret[DB as usize].unwrap() + ret[FD as usize].unwrap()); // x'
    ret[UR as usize] = Some(Cube::from(alg![U, D3]) + e3); // y
    ret[UB as usize] = Some(ret[UR as usize].unwrap() + ret[UR as usize].unwrap()); // y2
    ret[UL as usize] = Some(ret[UB as usize].unwrap() + ret[UR as usize].unwrap()); // y'
    ret[LF as usize] = Some(Cube::from(alg![F, B3]) + s); // z
    ret[DF as usize] = Some(ret[LF as usize].unwrap() + ret[LF as usize].unwrap()); // z2
    ret[RF as usize] = Some(ret[DF as usize].unwrap() + ret[LF as usize].unwrap()); // z'

    ret[DR as usize] = Some(ret[UL as usize].unwrap() + ret[DB as usize].unwrap());
    ret[DL as usize] = Some(ret[UR as usize].unwrap() + ret[DB as usize].unwrap());
    ret[RU as usize] = Some(ret[UL as usize].unwrap() + ret[BU as usize].unwrap());
    ret[LU as usize] = Some(ret[UR as usize].unwrap() + ret[BU as usize].unwrap());
    ret[LD as usize] = Some(ret[UL as usize].unwrap() + ret[FD as usize].unwrap());
    ret[RD as usize] = Some(ret[UR as usize].unwrap() + ret[FD as usize].unwrap());
    ret[BL as usize] = Some(ret[UL as usize].unwrap() + ret[LF as usize].unwrap());
    ret[FR as usize] = Some(ret[UR as usize].unwrap() + ret[LF as usize].unwrap());
    ret[FL as usize] = Some(ret[UL as usize].unwrap() + ret[RF as usize].unwrap());
    ret[BR as usize] = Some(ret[UR as usize].unwrap() + ret[RF as usize].unwrap());
    ret[FU as usize] = Some(ret[UB as usize].unwrap() + ret[BU as usize].unwrap());
    ret[BD as usize] = Some(ret[UB as usize].unwrap() + ret[FD as usize].unwrap());
    ret[RB as usize] = Some(ret[UB as usize].unwrap() + ret[LF as usize].unwrap());
    ret[LB as usize] = Some(ret[UB as usize].unwrap() + ret[RF as usize].unwrap());

    ret[URm as usize] = Some(ret[UR as usize].unwrap() + lr);
    ret[DRm as usize] = Some(ret[DR as usize].unwrap() + lr);
    ret[DLm as usize] = Some(ret[DL as usize].unwrap() + lr);
    ret[ULm as usize] = Some(ret[UL as usize].unwrap() + lr);
    ret[UFm as usize] = Some(ret[UF as usize].unwrap() + lr);
    ret[DFm as usize] = Some(ret[DF as usize].unwrap() + lr);
    ret[DBm as usize] = Some(ret[DB as usize].unwrap() + lr);
    ret[UBm as usize] = Some(ret[UB as usize].unwrap() + lr);
    ret[FRm as usize] = Some(ret[FR as usize].unwrap() + lr);
    ret[BRm as usize] = Some(ret[BR as usize].unwrap() + lr);
    ret[BLm as usize] = Some(ret[BL as usize].unwrap() + lr);
    ret[FLm as usize] = Some(ret[FL as usize].unwrap() + lr);
    ret[RUm as usize] = Some(ret[RU as usize].unwrap() + lr);
    ret[RDm as usize] = Some(ret[RD as usize].unwrap() + lr);
    ret[LDm as usize] = Some(ret[LD as usize].unwrap() + lr);
    ret[LUm as usize] = Some(ret[LU as usize].unwrap() + lr);
    ret[FUm as usize] = Some(ret[FU as usize].unwrap() + lr);
    ret[FDm as usize] = Some(ret[FD as usize].unwrap() + lr);
    ret[BDm as usize] = Some(ret[BD as usize].unwrap() + lr);
    ret[BUm as usize] = Some(ret[BU as usize].unwrap() + lr);
    ret[RFm as usize] = Some(ret[RF as usize].unwrap() + lr);
    ret[RBm as usize] = Some(ret[RB as usize].unwrap() + lr);
    ret[LBm as usize] = Some(ret[LB as usize].unwrap() + lr);
    ret[LFm as usize] = Some(ret[LF as usize].unwrap() + lr);

    ret.map(Option::unwrap)
});

/// Cayley table for the group of 48 cube symmetries
static SYM_ADD_TABLE: LazyLock<[[Sym; 48]; 48]> = LazyLock::new(|| {
    let syms: [Cube; 48] = Sym::all().map(|s| Cube::new().apply_sym(s));
    let table: [[Sym; 48]; 48] = array::from_fn(|i| {
        let a = syms[i];
        array::from_fn(|j| {
            let ab = a.apply_sym(Sym::from_u8(j as u8));
            let mut res: Option<Sym> = None;
            for k in 0..48 {
                if ab == syms[k] {
                    res = Some(Sym::from_u8(k as u8));
                    break;
                }
            }
            res.unwrap()
        })
    });
    table
});

static SYM_INVERSE_TABLE: LazyLock<[Sym; 48]> = LazyLock::new(|| {
    let syms: [Cube; 48] = Sym::all().map(|s| Cube::new().apply_sym(s));
    array::from_fn(|i| {
        let a = syms[i];
        let mut res: Option<Sym> = None;
        for j in 0..48 {
            let ab = a.apply_sym(Sym::from_u8(j as u8));
            if ab.is_solved() {
                res = Some(Sym::from_u8(j as u8));
            }
        }
        res.unwrap()
    })
});

// copies the edge enum and adds mirrors
// TODO this is kinda ugly because the identity is not 0
// a sym XY means we move sticker XY to UF
// XYm means do XY, and then mirror across M slice
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sym {
    UR = 0,
    DR = 1,
    DL = 2,
    UL = 3,
    UF = 4,
    DF = 5,
    DB = 6,
    UB = 7,
    FR = 8,
    BR = 9,
    BL = 10,
    FL = 11,
    RU = 12,
    RD = 13,
    LD = 14,
    LU = 15,
    FU = 16,
    FD = 17,
    BD = 18,
    BU = 19,
    RF = 20,
    RB = 21,
    LB = 22,
    LF = 23,
    URm = 24,
    DRm = 25,
    DLm = 26,
    ULm = 27,
    UFm = 28,
    DFm = 29,
    DBm = 30,
    UBm = 31,
    FRm = 32,
    BRm = 33,
    BLm = 34,
    FLm = 35,
    RUm = 36,
    RDm = 37,
    LDm = 38,
    LUm = 39,
    FUm = 40,
    FDm = 41,
    BDm = 42,
    BUm = 43,
    RFm = 44,
    RBm = 45,
    LBm = 46,
    LFm = 47,
}

impl Into<u8> for Sym {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Sym {
    fn from(x: u8) -> Self {
        Self::from_u8(x)
    }
}

impl Add<Self> for Sym {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SYM_ADD_TABLE[self as usize][rhs as usize]
    }
}

impl Sym {
    /// Whether the symmetry is a pure rotation (i.e. it doesn't involve
    /// a mirroring operation).
    pub const fn is_rotation(self) -> bool {
        (self as u8) < 24
    }

    pub const fn from_u8(x: u8) -> Self {
        debug_assert!(x < 48);
        unsafe { std::mem::transmute::<u8, Sym>(x) }
    }

    pub const fn all() -> [Sym; 48] {
        let mut ret = [Sym::UF; 48];
        let mut i = 0;
        while i < 48 {
            ret[i] = Sym::from_u8(i as u8);
            i += 1;
        }
        ret
    }

    pub fn inverse(self) -> Self {
        SYM_INVERSE_TABLE[self as usize]
    }

    pub fn cube(self) -> Cube {
        SYM_CUBE[self as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sym_add() {
        assert_eq!(Sym::UF, Sym::DR + Sym::DR);
        assert_eq!(Sym::UF, Sym::BR + Sym::UL + Sym::FD);
        assert_eq!(Sym::UF, Sym::BL + Sym::RF + Sym::DF + Sym::DL);
        assert_eq!(Sym::UR + Sym::FRm, Sym::RBm);
    }

    #[test]
    pub fn test_syms_unique() {
        for i in 0..48 {
            let a = Sym::from_u8(i);
            for j in (i + 1)..48 {
                let b = Sym::from_u8(j);
                assert_ne!(a.cube(), b.cube());
            }
        }
    }

    #[test]
    pub fn test_cayley_table_rows_bijective() {
        use std::collections::HashSet;
        for i in 0..48 {
            let s: HashSet<Sym> = HashSet::from_iter(SYM_ADD_TABLE[i]);
            assert_eq!(s.len(), 48);
        }
    }

    #[test]
    pub fn test_inverse() {
        for sym in Sym::all() {
            assert_eq!(sym + sym.inverse(), Sym::UF)
        }
    }
}
