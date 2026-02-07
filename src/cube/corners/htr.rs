use crate::*;

pub fn compute_corner_cosets() -> Vec<usize> {
    let mut s = Searcher::new(Cube::new(), Move::all());
    let mut cosets: Vec<Option<usize>> = vec![None; 40320];
    let mut count = 0;
    while let Some(c) = s.next() {
        let cp = c.corners.perm_coord();
        if cosets[cp].is_some() {
            s.prune();
            continue;
        }
        let mut s2 = Searcher::new(c, Move::htr_moveset());
        while let Some(c2) = s2.next() {
            let cp = c2.corners.perm_coord();
            if cosets[cp].is_some() {
                s2.prune();
                continue;
            }
            cosets[cp] = Some(count);
        }
        count += 1;
    }
    cosets.into_iter().map(Option::unwrap).collect()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_cp_htr_cosets() {
        let cosets = compute_corner_cosets();
        let c = Cube::new().apply_alg("R U".try_into().unwrap());
        println!("{}", cosets[c.corners.perm_coord()]);
        let mut s = Searcher::new(c, Move::all());
        while let Some(c) = s.next() {
            if s.moves().len() >= 7 || c.is_drud() {
                s.prune();
            }
            if c.is_drud() && cosets[c.corners.perm_coord()] != 0 {
                println!("{}", s.moves().alg());
            }
        }
    }
}
