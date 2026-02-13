use crate::Alg;
use crate::Cube;
use crate::Move;
use crate::Set64;

#[derive(Clone)]
struct Frame {
    moves: Set64<Move>,
    cube: Cube,
}

pub struct SearcherMoves<'a> {
    stack: &'a Vec<Frame>,
}

impl<'a> SearcherMoves<'a> {
    pub fn len(&self) -> usize {
        self.stack.len() - 1
    }

    pub fn get(&self, index: usize) -> Option<Move> {
        let frame = self.stack.get(index + 1)?;
        Some(frame.moves.first().unwrap())
    }

    pub fn index(&self, index: usize) -> Move {
        self.stack[index + 1].moves.first().unwrap()
    }

    pub fn alg(&self) -> Alg {
        self.stack
            .iter()
            .skip(1)
            .map(|x| x.moves.first().unwrap())
            .collect::<Alg>()
    }
}

#[derive(Clone)]
pub struct Searcher {
    stack: Vec<Frame>,
    succs: [Set64<Move>; 18],
    moveset: Set64<Move>,
    start: Cube,
    pruned: bool,
}

impl Default for Searcher {
    fn default() -> Self {
        Self::new(Cube::default(), Move::all())
    }
}

impl Searcher {
    pub fn new(start: Cube, moveset: impl IntoIterator<Item = Move>) -> Self {
        let moveset = moveset.into_iter().collect::<Set64<_>>();
        Searcher {
            start,
            moveset: moveset.clone(),
            stack: Vec::new(),
            succs: std::array::from_fn(|i| {
                let m = Move::from(i as u8);
                moveset
                    .into_iter()
                    .filter(|next| next.canonically_succeeds(m))
                    .collect::<Set64<_>>()
            }),
            pruned: false,
        }
    }

    pub fn moves(&self) -> SearcherMoves<'_> {
        SearcherMoves { stack: &self.stack }
    }

    pub fn prune(&mut self) {
        self.pruned = true;
    }

    pub fn reset(&mut self) {
        *self = Searcher::new(self.start, self.moveset);
    }
}

impl Iterator for Searcher {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        // Not pruned, push a frame
        if !self.pruned {
            let frame = match self.stack.as_slice() {
                [] => Frame {
                    moves: Set64::new(),
                    cube: self.start,
                },
                [top] => Frame {
                    moves: self.moveset,
                    cube: top.cube.apply_move(self.moveset.first()?),
                },
                [.., _, top] => {
                    let moves = self.succs[top.moves.first()? as usize];
                    Frame {
                        moves,
                        cube: top.cube.apply_move(moves.first()?),
                    }
                }
            };
            let cube = frame.cube;
            self.stack.push(frame);
            return Some(cube);
        }

        // Pruned, increment the current frame and pop up the stack as necessary
        self.pruned = false;
        loop {
            let (prev, top) = match self.stack.as_mut_slice() {
                [.., prev, top] => (prev, top),
                _ => {
                    return None;
                }
            };
            top.moves.pop_first();
            match top.moves.first() {
                None => {
                    self.stack.pop();
                }
                Some(m) => {
                    top.cube = prev.cube.apply_move(m);
                    return Some(top.cube);
                }
            }
        }
    }
}

/// Thin wrapper around `Searcher` that yields results in order of length.
#[derive(Default)]
pub struct IDASearcher {
    pub n: usize,
    pub searcher: Searcher,
}

impl IDASearcher {
    pub fn new(start: Cube) -> Self {
        Self {
            n: 0,
            searcher: Searcher::new(start, Move::all()),
        }
    }

    pub fn new_with_moveset(start: Cube, moveset: impl IntoIterator<Item = Move>) -> Self {
        Self {
            n: 0,
            searcher: Searcher::new(start, moveset),
        }
    }

    pub fn moves(&self) -> SearcherMoves<'_> {
        self.searcher.moves()
    }

    pub fn prune(&mut self) {
        self.searcher.pruned = true;
    }

    pub fn is_frontier(&self) -> bool {
        self.searcher.moves().len() == self.n
    }

    pub fn next_frontier(&mut self) -> Option<Cube> {
        while let Some(c) = self.next() {
            if !self.is_frontier() {
                continue;
            }
            return Some(c);
        }
        None
    }
}

impl Iterator for IDASearcher {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.searcher.next() {
            if self.searcher.moves().len() == self.n {
                self.searcher.prune()
            }
            Some(c)
        } else {
            self.n += 1;
            self.searcher.reset();
            self.searcher.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_foobar() {
        let mut s = IDASearcher::default();
        while let Some(_c) = s.next() {
            if !s.is_frontier() {
                continue;
            }
            if s.moves().len() >= 3 {
                break;
            }
            println!("{}", s.moves().alg());
        }
    }

    #[test]
    pub fn test_canonical_distribution() {
        let mut s = Searcher::default();
        let mut dist = [0; 6];
        while s.next().is_some() {
            if s.moves().len() == 5 {
                s.prune();
            }
            dist[s.moves().len()] += 1;
        }
        // https://tomas.rokicki.com/rubik20.pdf
        assert_eq!(dist, [1, 18, 243, 3240, 43254, 577368]);
    }

    #[test]
    pub fn test_find_eofb() {
        let c = Cube::default().apply_alg(
            Alg::try_from("R' U' F R F' R' B' L' D' B2 L' D B2 R' B2 D2 B2 L' F2 L U2 D2 R' U' F")
                .unwrap(),
        );
        let pt: PruneTable<CoordEOFB> = PruneTable::new();
        let mut s = Searcher::new(c, Move::all());
        let mut count = 0;
        while let Some(c) = s.next() {
            let n = s.moves().len();
            if n + pt.eval(c) > 7 {
                s.prune();
                continue;
            }
            if c.is_eofb() {
                s.prune();
            }
            let is_canonical = (|| {
                let m = if n < 1 {
                    return true;
                } else {
                    s.moves().index(n - 1)
                };
                if !m.is_clockwise() {
                    return false;
                }
                let m2 = if n < 2 {
                    return true;
                } else {
                    s.moves().index(n - 2)
                };
                !m2.commutes_with(m) || m2.is_clockwise()
            })();
            if c.is_eofb() && is_canonical {
                count += 1;
            }
        }
        println!("found {} eos", count);
    }

    #[test]
    pub fn test_foo() {
        let c = Cube::default().apply_alg(
            Alg::try_from("R' U' F R F' R' B' L' D' B2 L' D B2 R' B2 D2 B2 L' F2 L U2 D2 R' U' F")
                .unwrap(),
        );

        let c2 = c.apply_alg("R U L F B".try_into().unwrap());
        println!("{:?}", c2.edges);
    }
}
