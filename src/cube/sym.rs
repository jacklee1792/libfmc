use crate::*;

// copies the edge enum and adds mirrors
// a sym XY means we move sticker XY to UF
// XYm means do XY, and then mirror across M slice
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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

impl Sym {
    pub const fn inverse(self) -> Self {
        use Sym::*;
        match self {
            UR => UL,
            DR => DR,
            DL => DL,
            UL => UR,
            UF => UF,
            DF => DF,
            DB => DB,
            UB => UB,
            FR => RU,
            BR => LD,
            BL => RD,
            FL => LU,
            RU => FR,
            RD => BL,
            LD => BR,
            LU => FL,
            FU => FU,
            FD => BD,
            BD => BD,
            BU => FD,
            RF => LF,
            RB => RB,
            LB => LB,
            LF => RF,
            URm => ULm,
            DRm => DLm,
            DLm => DRm,
            ULm => URm,
            UFm => UFm,
            DFm => DFm,
            DBm => DBm,
            UBm => UBm,
            FRm => LUm,
            BRm => RDm,
            BLm => LDm,
            FLm => RUm,
            RUm => FLm,
            RDm => BRm,
            LDm => BLm,
            LUm => FRm,
            FUm => FUm,
            FDm => BUm,
            BDm => BDm,
            BUm => FDm,
            RFm => RFm,
            RBm => LBm,
            LBm => RBm,
            LFm => LFm,
        }
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
                Corners::cycle_drud([Corner::UFL, Corner::UFR])
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
            URm => panic!(),
            DRm => panic!(),
            DLm => panic!(),
            ULm => panic!(),
            UFm => panic!(),
            DFm => panic!(),
            DBm => panic!(),
            UBm => panic!(),
            FRm => panic!(),
            BRm => panic!(),
            BLm => panic!(),
            FLm => panic!(),
            RUm => panic!(),
            RDm => panic!(),
            LDm => panic!(),
            LUm => panic!(),
            FUm => panic!(),
            FDm => panic!(),
            BDm => panic!(),
            BUm => panic!(),
            RFm => panic!(),
            RBm => panic!(),
            LBm => panic!(),
            LFm => panic!(),
        }
    }
}
