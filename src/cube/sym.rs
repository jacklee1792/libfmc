use crate::*;
use crate::cube::corners;
use std::ops::Add;
use std::sync::LazyLock;
use std::array;

/// Cayley table for the group of 48 cube symmetries
static SYM_ADD_TABLE: LazyLock<[[Sym; 48]; 48]> = LazyLock::new(|| {
    let syms: [Cube; 48] = Sym::all().map(|s| {
        Cube::new().apply_sym(s)
    });
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
    let syms: [Cube; 48] = Sym::all().map(|s| {
        Cube::new().apply_sym(s)
    });
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
        use Sym::*;
        use Move::*;
        let m_edges = [Edge::UF, Edge::UB, Edge::DB, Edge::DF];
        let e_edges = [Edge::FR, Edge::FL, Edge::BL, Edge::BR];
        let s_edges = [Edge::UR, Edge::DR, Edge::DL, Edge::UL];
        let m3 = Cube::from(Edges::cycle_eolr(m_edges) + Edges::flip(m_edges));
        let e3 = Cube::from(Edges::cycle_eoud(e_edges) + Edges::flip(e_edges));
        let s = Cube::from(Edges::cycle_eolr(s_edges) + Edges::flip(s_edges));

        let lr = Cube {
            edges: (
                Edges::cycle_eofb([Edge::UL, Edge::UR]) +
                Edges::cycle_eofb([Edge::DL, Edge::DR]) +
                Edges::cycle_eofb([Edge::FL, Edge::FR]) +
                Edges::cycle_eofb([Edge::BL, Edge::BR])
            ),
            corners: (
                Corners::cycle_drud([Corner::UFL, Corner::UFR]) +
                Corners::cycle_drud([Corner::UBL, Corner::UBR]) +
                Corners::cycle_drud([Corner::DFL, Corner::DFR]) +
                Corners::cycle_drud([Corner::DBL, Corner::DBR])
            )
        };
        
        match self {
            UF => Cube::default(),
            FD => Cube::from(alg![R, L3]) + m3, // x
            DB => FD.cube() + FD.cube(), // x2
            BU => FD.cube() + FD.cube() + FD.cube(), // x'
            UR => Cube::from(alg![U, D3]) + e3, // y
            UB => UR.cube() + UR.cube(), // y2
            UL => UR.cube() + UR.cube() + UR.cube(), // y'
            LF => Cube::from(alg![F, B3]) + s, // z
            DF => LF.cube() + LF.cube(), // z2
            RF => LF.cube() + LF.cube() + LF.cube(), // z'
            DR => UL.cube() + DB.cube(),
            DL => UR.cube() + DB.cube(),
            RU => UL.cube() + BU.cube(),
            LU => UR.cube() + BU.cube(),
            LD => UL.cube() + FD.cube(),
            RD => UR.cube() + FD.cube(),
            BL => UL.cube() + LF.cube(),
            FR => UR.cube() + LF.cube(),
            FL => UL.cube() + RF.cube(),
            BR => UR.cube() + RF.cube(),
            FU => UB.cube() + BU.cube(),
            BD => UB.cube() + FD.cube(),
            RB => UB.cube() + LF.cube(),
            LB => UB.cube() + RF.cube(),
            URm => UR.cube() + lr,
            DRm => DR.cube() + lr,
            DLm => DL.cube() + lr,
            ULm => UL.cube() + lr,
            UFm => UF.cube() + lr,
            DFm => DF.cube() + lr,
            DBm => DB.cube() + lr,
            UBm => UB.cube() + lr,
            FRm => FR.cube() + lr,
            BRm => BR.cube() + lr,
            BLm => BL.cube() + lr,
            FLm => FL.cube() + lr,
            RUm => RU.cube() + lr,
            RDm => RD.cube() + lr,
            LDm => LD.cube() + lr,
            LUm => LU.cube() + lr,
            FUm => FU.cube() + lr,
            FDm => FD.cube() + lr,
            BDm => BD.cube() + lr,
            BUm => BU.cube() + lr,
            RFm => RF.cube() + lr,
            RBm => RB.cube() + lr,
            LBm => LB.cube() + lr,
            LFm => LF.cube() + lr,
        }
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
            for j in (i+1)..48 {
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
