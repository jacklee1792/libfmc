use crate::Cube;
use crate::Move;
use crate::Set32;
use crate::Alg;

struct Frame {
    moves: Set32<Move>,
    cube: Cube,
}

pub struct Searcher {
    stack: Vec<Frame>,
    succs: [Set32<Move>; 18],
    moveset: Set32<Move>,
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
        let moveset = moveset.into_iter().collect::<Set32<_>>();
        Searcher {
            start,
            moveset: moveset.clone(),
            stack: Vec::new(),
            succs: std::array::from_fn(|i| {
                let m = Move::from(i as u8);
                moveset
                    .into_iter()
                    .filter(|next| next.canonically_succeeds(m))
                    .collect::<Set32<_>>()
            }),
            pruned: false,
        }
    }

    pub fn moves(&self) -> Alg {
        self.stack
            .iter()
            .skip(1)
            .map(|x| x.moves.first().unwrap())
            .collect::<Alg>()
    }

    pub fn nth(&self, n: usize) -> Option<Move> {
        if n >= self.len() {
            None
        } else {
            Some(self.stack[n + 1].moves.first().unwrap())
        }
    }

    pub fn nth_last(&self, n: usize) -> Option<Move> {
        if n >= self.len() {
            None
        } else {
            Some(self.stack[self.len() - n].moves.first().unwrap())
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len() - 1
    }

    pub fn prune(&mut self) {
        self.pruned = true;
    }
}

impl Iterator for Searcher {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        // Not pruned, push a frame
        if !self.pruned {
            let frame = match self.stack.as_slice() {
                [] => Frame {
                    moves: Set32::new(),
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
                [.., prev, top] => {
                    (prev, top)
                }
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_canonical_distribution() {
        let mut s = Searcher::default();
        let mut dist = [0; 6];
        while s.next().is_some() {
            if s.len() == 5 {
                s.prune();
            }
            dist[s.len()] += 1;
        }
        // https://tomas.rokicki.com/rubik20.pdf
        assert_eq!(dist, [1, 18, 243, 3240, 43254, 577368]);
    }

    #[test]
    pub fn test_find_eofb() {
        let c = Cube::default().apply_alg(Alg::try_from("R' U' F R F' R' B' L' D' B2 L' D B2 R' B2 D2 B2 L' F2 L U2 D2 R' U' F").unwrap());
        let mut s = Searcher::new(c, Move::all());
        let mut count = 0;
        while let Some(c) = s.next() {
            if s.len() == 7 || c.is_eofb() {
                s.prune();
            }
            let is_canonical = (|| {
                let m = match s.nth_last(0) {
                    None => return true,
                    Some(m) => m
                };
                if !m.is_clockwise() {
                    return false;
                }
                let m2 = match s.nth_last(1) {
                    None => return true,
                    Some(m2) => m2
                };
                !m2.commutes_with(m) || m2.is_clockwise()
            })();
            if c.is_eofb() && is_canonical {
                println!("{}", s.moves());
                count += 1;
            }
        }
        println!("found {} eos", count);
    }

    #[test]
    pub fn test_foo() {
        let c = Cube::default().apply_alg(Alg::try_from("R' U' F R F' R' B' L' D' B2 L' D B2 R' B2 D2 B2 L' F2 L U2 D2 R' U' F").unwrap());

        use Move::*;
        let c2 = c.apply_alg("R U L F B".try_into().unwrap());
        println!("{:?}", c2.edges);
    }
}
