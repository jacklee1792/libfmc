use crate::*;

/// Coordinate based on EO of 11 edges [0..2^11)
pub struct CoordEOFB;

impl Coord for CoordEOFB {
    const NAME: &'static str = "CoordEO";
    const N_VALUES: usize = 2048;
    const SYMS: &[Sym] = &[
        Sym::UF,
        Sym::UB,
        Sym::DF,
        Sym::DB,
        Sym::UFm,
        Sym::UBm,
        Sym::DFm,
        Sym::DBm,
    ];

    fn index(c: Cube) -> usize {
        c.edges.index_eofb()
    }

    fn unindex(coord: usize) -> Cube {
        Cube {
            edges: Edges::unindex_eofb(coord),
            corners: Corners::new(),
        }
    }
}
