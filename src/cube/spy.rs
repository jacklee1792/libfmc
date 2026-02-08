use crate::*;
use std::io::{BufRead, Write};

pub struct Spy {
    cube: Cube,
    stack: Alg,
}

impl Spy {
    pub fn new() -> Self {
        let mut scramble = String::new();
        let stdin = std::io::stdin();
        print!("Enter scramble: ");
        stdin.lock().read_line(&mut scramble).unwrap();
        let scramble = Alg::try_from(scramble.as_str()).unwrap();
        let inverse = Cube::default().apply_alg(scramble);
        Spy {
            cube: Cube::default(),
            stack: Alg::new(),
        }
    }

    pub fn handle(&mut self, m: Move) {
        println!("hi");
        self.stack.push(m);
        self.stack = self.stack.canonicalize();
        self.cube = self.cube.apply_move(m);

        let eofb_count = Edge::all()
            .into_iter()
            .filter(|e| self.cube.edges.eofb(*e) == EO::Flipped)
            .count();
        let eolr_count = Edge::all()
            .into_iter()
            .filter(|e| self.cube.edges.eolr(*e) == EO::Flipped)
            .count();
        let eoud_count = Edge::all()
            .into_iter()
            .filter(|e| self.cube.edges.eoud(*e) == EO::Flipped)
            .count();

        let coud_count = Corner::all()
            .into_iter()
            .filter(|c| self.cube.corners.coud(*c) != CO::Solved)
            .count();
        let colr_count = Corner::all()
            .into_iter()
            .filter(|c| self.cube.corners.colr(*c) != CO::Solved)
            .count();
        let cofb_count = Corner::all()
            .into_iter()
            .filter(|c| self.cube.corners.cofb(*c) != CO::Solved)
            .count();
        let drud = match (eofb_count != 0, eolr_count != 0) {
            (true, true) => "----".to_owned(),
            (true, false) => format!("{}c{}e", coud_count, eofb_count),
            (false, true) => format!("{}c{}e", coud_count, eolr_count),
            (false, false) => {
                if coud_count == 0 {
                    "done".to_owned()
                } else {
                    format!("{}c", coud_count)
                }
            }
        };
        let drlr = match (eofb_count != 0, eoud_count != 0) {
            (true, true) => "----".to_owned(),
            (true, false) => format!("{}c{}e", colr_count, eofb_count),
            (false, true) => format!("{}c{}e", colr_count, eoud_count),
            (false, false) => {
                if colr_count == 0 {
                    "done".to_owned()
                } else {
                    format!("{}c", colr_count)
                }
            }
        };
        let drfb = match (eolr_count != 0, eoud_count != 0) {
            (true, true) => "----".to_owned(),
            (true, false) => format!("{}c{}e", cofb_count, eolr_count),
            (false, true) => format!("{}c{}e", cofb_count, eoud_count),
            (false, false) => {
                if cofb_count == 0 {
                    "done".to_owned()
                } else {
                    format!("{}c", cofb_count)
                }
            }
        };

        let n_lines = 3;
        println!(
            "\r\x1b[2Keofb: {:<4}  eolr: {:<4}  eoud: {:<4}",
            eofb_count, eolr_count, eoud_count
        );
        println!(
            "\r\x1b[2Kdrud: {:<4}  drlr: {:<4}  drfb: {:<4}",
            drud, drlr, drfb
        );
        println!("\r\x1b[2K{}", Alg::from(self.stack.clone()));
        print!("\x1b[{}A", n_lines);
        std::io::stdout().flush().unwrap();

        // println!("eofb: {:<4}  eolr: {:<4}  eoud: {:<4}", eofb_count, eolr_count, eoud_count);
        // println!("drud: {:<4}  drlr: {:<4}  drfb: {:<4}", drud, drlr, drfb);
        // println!("{}", Alg::from(self.stack.clone()));

        println!("==== edges =====");
        println!("{:?}", self.cube.edges);
        println!("==== corners =====");
        println!("{:?}", self.cube.corners);
    }
}
