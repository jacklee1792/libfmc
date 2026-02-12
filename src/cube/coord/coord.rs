use crate::*;
use std::marker::PhantomData;

/// Description of a coordinate which maps to a (right) coset of the cube.
pub trait Coord {
    /// Name of the coordinate, used for debugging purposes.
    const NAME: &'static str;

    /// The number of values of the coordinate.
    const N_VALUES: usize;

    /// Listing of symmetries which are applicable to the coordinate, i.e. symmetries
    /// `S` which have the following property: if `x`, `y` are cubes with the same coordinate, then
    /// the of `sxs'`, `sys'` also have the same coordinate for all `s` in `S`.
    const SYMS: &[Sym];

    /// Inspect a cube and produce a coordinate associated with the it, some number in
    /// [0, N_VALUES).
    fn index(c: Cube) -> usize;

    /// Given a coordinate, produce a cube which has that coordinate when `index` is
    /// called on it.
    fn unindex(c: usize) -> Cube;
}

/// Symmetry information about a coordinate `C`.
pub struct CoordSyms<C>
where
    C: Coord,
{
    /// For each coordinate, the conjugacy class it belongs to.
    coord_cls: Vec<usize>,

    /// For each coordinate, any symmetry which brings the coordinate
    /// back to the canonical representative via conjugation.
    coord_canonicalizer: Vec<Sym>,

    /// For each conjugacy class, the coordinate which is the canonical
    /// representative for that class.
    class_rep: Vec<usize>,

    /// For each conjugacy class, the set of self-symmetries for that
    /// class' canonical representative.
    class_ssym: Vec<Set64<Sym>>,

    /// Phantom type to associate with the original Coord type
    _c: PhantomData<C>,
}

impl<C> CoordSyms<C>
where
    C: Coord,
{
    pub fn new() -> Self {
        let mut coord_canonicalizer: Vec<Option<Sym>> = vec![None; C::N_VALUES];
        let mut coord_cls: Vec<usize> = vec![0; C::N_VALUES];
        let mut class_rep: Vec<usize> = vec![];
        let mut class_ssym: Vec<Set64<Sym>> = vec![];

        let mut clsno = 0;
        for a_coord in 0..C::N_VALUES {
            if coord_canonicalizer[a_coord].is_some() {
                continue;
            }
            // Otherwise, a is a newly discovered conjugacy class, mark
            // all coordinates in the same class
            let a = C::unindex(a_coord);
            let mut ssyms = Set64::<Sym>::new();
            for sym in C::SYMS {
                let b = a.conjugate_sym(*sym);
                let b_coord = C::index(b);
                // sas' = b => s'bs = a
                coord_canonicalizer[b_coord] = Some(sym.inverse());
                coord_cls[b_coord] = clsno;
                if a_coord == b_coord {
                    ssyms.insert(*sym);
                }
            }
            clsno += 1;
            class_rep.push(a_coord);
            class_ssym.push(ssyms);
        }

        Self {
            coord_canonicalizer: coord_canonicalizer
                .into_iter()
                .map(Option::unwrap)
                .collect(),
            coord_cls: coord_cls,
            class_rep: class_rep,
            class_ssym: class_ssym,
            _c: PhantomData,
        }
    }

    /// Number of conjugacy classes for the coordinate.
    pub fn n_classes(&self) -> usize {
        self.class_rep.len()
    }

    /// Symmetry which brings the coordinate to its conjugacy class representative.
    pub fn canonicalizer(&self, coord: usize) -> Sym {
        self.coord_canonicalizer[coord]
    }

    /// Conjugacy class of the given coordinate.
    pub fn conjugacy_class(&self, coord: usize) -> usize {
        self.coord_cls[coord]
    }

    /// Canonicalize the given cube, applying a transformation so that coordinate is the
    /// representative of its conjugacy class.
    pub fn canonicalize(&self, c: Cube) -> Cube {
        let coord = C::index(c);
        let sym = self.canonicalizer(coord);
        c.conjugate_sym(sym)
    }

    pub fn self_syms(&self, coord: usize) -> Set64<Sym> {
        let cls = self.coord_cls[coord];
        self.class_ssym[cls]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_coordsyms_eofb() {
        let c = CoordSyms::<CoordEOFB>::new();
        assert_eq!(c.n_classes(), 336);
        for i in 0..2048 {
            let cube = CoordEOFB::unindex(i);
            for s in c.self_syms(i).into_iter() {
                assert_eq!(
                    CoordEOFB::index(cube),
                    CoordEOFB::index(cube.conjugate_sym(s))
                );
            }
        }
        assert_eq!(CoordEOFB::index(Cube::default()), 0);
    }

    #[test]
    pub fn test_coordsyms_coud() {
        let c = CoordSyms::<CoordCOUD>::new();
        assert_eq!(c.n_classes(), 168);
        assert_eq!(CoordCOUD::index(Cube::default()), 0);
    }
}
