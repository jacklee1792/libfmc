use crate::*;

/// Coordinate based on CO of 7 corners [0..3^7)
pub struct CoordCOUD;

impl Coord for CoordCOUD {
    const NAME: &'static str = "CoordCO";
    const N_VALUES: usize = 2187;
    const SYMS: &[Sym] = &[
        Sym::UF,
        Sym::UR,
        Sym::UL,
        Sym::UB,
        Sym::DF,
        Sym::DR,
        Sym::DL,
        Sym::DB,
        Sym::UFm,
        Sym::URm,
        Sym::ULm,
        Sym::UBm,
        Sym::DFm,
        Sym::DRm,
        Sym::DLm,
        Sym::DBm,
    ];

    fn index(c: Cube) -> usize {
        c.corners.index_coud()
    }

    fn unindex(coord: usize) -> Cube {
        Cube {
            edges: Edges::new(),
            corners: Corners::unindex_coud(coord),
        }
    }
}
