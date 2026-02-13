use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::*;

/// Pruning table for a composite coordinate (R, C), where R is reduced by symmetry.
pub struct PruneTable<R, C>
where
    R: Coord,
    C: Coord,
{
    rsym: CoordSyms<R>,
    dist: Vec<u8>,
    _r: PhantomData<R>,
    _c: PhantomData<C>,
}

impl<R, C> PruneTable<R, C>
where
    R: Coord,
    C: Coord,
{
    pub fn new() -> Self {
        let rsym = CoordSyms::new();
        let mut dist = vec![None; rsym.n_classes() * C::N_VALUES];
        let mut q: VecDeque<(Cube, u8)> = VecDeque::new();
        dist[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((a, d)) = q.pop_front() {
            for m in Move::all() {
                let b = rsym.canonicalize(a.apply_move(m));
                let b_coord = R::index(b);
                let b_class = rsym.conjugacy_class(b_coord);
                for s in rsym.self_syms(b_coord) {
                    let b = b.conjugate_sym(s);
                    let coord = b_class * C::N_VALUES + C::index(b);
                    if dist[coord].is_none() {
                        dist[coord] = Some(d + 1);
                        q.push_back((b, d + 1));
                    }
                }
            }
        }
        let dist = dist.into_iter().map(Option::unwrap).collect::<Vec<_>>();
        Self {
            rsym,
            dist,
            _r: PhantomData,
            _c: PhantomData,
        }
    }

    /// Given a cube, produce a lower bound on the number of moves to reduce the coordinate to 0.
    pub fn eval(&self, c: Cube) -> usize {
        let c = self.rsym.canonicalize(c);
        let r_coord = self.rsym.conjugacy_class(R::index(c));
        let c_coord = C::index(c);
        let coord = r_coord * C::N_VALUES + c_coord;
        self.dist[coord] as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_prune() {
        let mut s = IDASearcher::new(Cube::default());
        let pt = PruneTable::<CoordEOFB, CoordCOUD>::new();
        while let Some(c) = s.next_frontier() {
            if s.moves().len() > 4 {
                break;
            }
            assert!(
                pt.eval(c) <= s.moves().len(),
                "moves = {}, pt = {}",
                s.moves().alg(),
                pt.eval(c),
            );
        }
    }

    #[test]
    fn test_solve_dr() {
        let pt = PruneTable::<CoordEOFB, CoordCOUD>::new();
        let alg = Alg::try_from(
            "R' U' F B2 R2 B2 F L2 B D2 B' L2 R2 D2 F' D L' B F' D L' B' L2 B2 R' U' F",
        )
        .unwrap();
        let start = Cube::from_alg(alg);
        let mut s = IDASearcher::new(start);
        while let Some(c) = s.next() {
            if s.n == 13 {
                break;
            }
            if s.is_frontier() && s.n != 0 && c.is_drud() {
                let last = *s.moves().alg().0.last().unwrap();
                let mut ok = last.is_clockwise() && !last.commutes_with(Move::U);
                if let Some([a, b]) = s.moves().alg().0.last_chunk::<2>() {
                    if a.commutes_with(*b) && !a.is_clockwise() {
                        ok = false;
                    }
                }
                if ok {
                    println!("found {}", s.moves().alg());
                }
                // println!("{:?}", c);
            }
            if s.moves().len() + pt.eval(c) as usize > s.n {
                s.prune();
            }
        }
    }
}
