use std::io::{self, BufRead};
use std::cmp::*;

impl<T, I: Iterator<Item=T>> Iterator for Pairs<T, I> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.0.next()?;
        let b = self.0.next()?;
        Some((a, b))
    }
}

fn estima(h: u64, c: u64, mut b: u64, mut p: u64, mut f: u64) -> u64 {
    let caro = max(h,c);
    let mut valor = 0;
    loop {
        if caro == h {
            if p > 0 && b > 1 {
                b-=2;
                p-=1;
                valor += h;
            } else if f > 0 && b > 1 {
                b-=2;
                f-=1;
                valor += c;
            } else {
                return valor;
            }
        } else {
            if f > 0 && b > 1 {
                b-=2;
                f-=1;
                valor += c;
            } else if p > 0 && b > 1 {
                b-=2;
                p-=1;
                valor += h;
            } else {
                return valor;
            }
        }
    }
}

#[test]
fn testa_estima() {
    let r = estima(5, 10, 15, 2, 3);
    assert_eq!(r, 40);
}

fn main() {
        for in Pairs(io::stdin().lock().lines().skip(1)) {
        let mut split = line.unwrap().split_whitespace();
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        println!("{}", value(x, y));
    }
}
