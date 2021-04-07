use std::io::{BufRead, BufReader};

fn read_input() -> usize {
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

    fn mov(
        from: &mut (&mut Vec<usize>, &str),
        to: &mut (&mut Vec<usize>, &str),
        aux: &mut (&mut Vec<usize>, &str),
        n: usize,
    ) {
        if n <= 1 {
            to.0.push(from.0.pop().unwrap());
            println!("{} {}", from.1, to.1);
            return;
        }

        HanoiTower::mov(from, aux, to, n - 1);
        HanoiTower::mov(from, to, aux, 1); // mov new layer
        HanoiTower::mov(aux, to, from, n - 1);
    }

    fn solve(&mut self) {
        let len = self.len();
        return HanoiTower::mov(
            &mut (&mut self.left, "1"),
            &mut (&mut self.right, "3"),
            &mut (&mut self.middle, "2"),
            len,
        );
    }
}

fn main() {
    let n = read_input();
    println!("{}", 2_u64.pow(n as u32) - 1);
    let mut tower = HanoiTower::new(n);
    tower.solve();
}
