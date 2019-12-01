use std::io;
use std::io::prelude::*;

fn fuel_computation(mut i: u32) -> u32 {
    let mut total = 0;
    while i > 0 {
        i = (i / 3).saturating_sub(2);
        total += i as u32;
    }
    total
}

fn main() {
    let stdin = io::stdin();
    let requirement: u32 = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .map(fuel_computation)
        .sum();
    println!("{}", requirement);
}
