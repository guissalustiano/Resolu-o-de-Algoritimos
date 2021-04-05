use std::cmp;
use std::io::{BufRead, BufReader};

fn read_input() -> u64 {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::from("");
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    n
}

fn main() {
    let n = read_input();
    let mut count2 = 0;
    let mut count5 = 0;
    let mut i = 1;
    while 5_u64.pow(i as u32) <= n {
        count2 += n / 2_u64.pow(i as u32);
        count5 += n / 5_u64.pow(i as u32);
        i += 1;
    }
    println!("{}", cmp::min(count2, count5));
}
