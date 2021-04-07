use std::io::{BufRead, BufReader};

fn read_input() -> u32 {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::from("");
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    n
}

fn to_gray(n: u64) -> u64 {
    n ^ n >> 1
}

fn main() {
    let n = read_input();
    for i in 0..2_u64.pow(n) {
        let s = format!("{:016b}", to_gray(i));
        let s = s.chars().skip(16 - n as usize).collect::<String>();
        println!("{}", s);
    }
}
