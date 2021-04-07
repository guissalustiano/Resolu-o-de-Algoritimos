use std::cmp::max;
use std::io::{self, BufRead};

#[inline]
// PA de 2° ordem
fn diagonal_value(n: isize) -> isize {
    (1..n).map(|x| 2 * x).sum::<isize>() + 1
}

fn value(x: isize, y: isize) -> isize {
    let bigger = max(x, y);
    if bigger % 2 == 0 {
        diagonal_value(bigger) + (x - y)
    } else {
        diagonal_value(bigger) - (x - y)
    }
}

fn main() {
    for line in io::stdin().lock().lines().skip(1) {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        println!("{}", value(x, y));
    }
}
