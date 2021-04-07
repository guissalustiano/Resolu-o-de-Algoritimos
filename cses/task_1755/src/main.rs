use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::from("");
    input.read_line(&mut line).unwrap();
    let mut s = line.trim().to_uppercase().chars().collect::<Vec<char>>();
    s.sort();

    let mut sobrou: Option<char> = None;
    let mut acc = Vec::<char>::with_capacity(s.len() / 2 + 2);
    let mut i = 0;
    while i < s.len() {
        if i + 1 < s.len() && s[i] == s[i + 1] {
            acc.push(s[i]);
            i += 2;
        } else {
            match sobrou {
                None => sobrou = Some(s[i]),
                Some(_) => {
                    println!("NO SOLUTION");
                    exit(0);
                }
            }
            i += 1;
        }
    }
    let text = acc.iter().collect::<String>();
    let rtext = acc.iter().rev().collect::<String>();
    print!("{}", text);
    if let Some(r) = sobrou {
        print!("{}", r);
    }
    println!("{}", rtext);
}
