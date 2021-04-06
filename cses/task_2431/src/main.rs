use std::io::{self, BufRead};

#[inline]
fn pow10(n: u32) -> u64 {
    10_u64.pow(n)
}

fn solve(k: u64) {
    let mut dig_acc = 0;
    let mut i = 1;
    while dig_acc < k {
        dig_acc += 9 * pow10(i - 1) * (i as u64);
        i += 1;
    }
    i -= 1;
    let mut num_acc = pow10(i) - 1;
    let dig_diff = dig_acc - k;
    let num_diff = dig_diff / (i as u64);
    let num_rest = dig_diff % (i as u64);
    num_acc -= num_diff;
    let dig = num_acc
        .to_string()
        .chars()
        .rev() // calculando voltando
        .nth(num_rest as usize)
        .unwrap();
    println!("{}", dig);
}

fn main() {
    for k in io::stdin().lock().lines().skip(1) {
        let k = k.unwrap().parse().unwrap();
        solve(k);
    }
}
