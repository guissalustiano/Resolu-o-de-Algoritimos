use std::io::{BufRead, BufReader};

fn _read_input() -> usize {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::from("");
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    n
}

#[derive(Debug, PartialEq, Eq)]
struct HanoiTower {
    left: Vec<usize>,
    middle: Vec<usize>,
    right: Vec<usize>,
}

impl HanoiTower {
    fn new(n: usize) -> HanoiTower {
        let mut tower = HanoiTower {
            left: Vec::new(),
            middle: Vec::new(),
            right: Vec::new(),
        };
        for i in (0..n).rev() {
            tower.left.push(i);
        }
        tower
    }
    fn len(&self) -> usize {
        self.left.len() + self.middle.len() + self.right.len()
    }

    fn mov_one(from: &mut Vec<usize>, to: &mut Vec<usize>) -> usize {
        to.push(from.pop().unwrap());
        return 1;
    }

    fn mov(from: &mut Vec<usize>, to: &mut Vec<usize>, aux: &mut Vec<usize>, n: usize) -> usize {
        if n <= 1 {
            return HanoiTower::mov_one(from, to);
        }

        HanoiTower::mov(from, aux, to, n - 1) +
        HanoiTower::mov_one(from, to) + // mov new layer
        HanoiTower::mov(aux, to, from, n - 1)
    }

    fn solve(&mut self) -> usize {
        let len = self.len();
        return HanoiTower::mov(&mut self.left, &mut self.middle, &mut self.right, len);
    }
}

fn main() {
    let n = 10; // read_input();
    let mut tower = HanoiTower::new(n);
    let n_moves = tower.solve();
    println!("{}", n_moves);
    println!("{:?}", tower);
}
