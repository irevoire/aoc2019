use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let requirement: i32 = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .map(|i| i / 3 - 2)
        .sum();
    println!("{}", requirement);
}
