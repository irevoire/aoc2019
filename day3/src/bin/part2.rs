use std::io;
use std::io::prelude::*;

use day3::*;

fn main() {
    let stdin = io::stdin();
    let paths: Vec<Path> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<Path>().unwrap())
        .collect();
    let (coord1, coord2) = (paths[0].convert(), paths[1].convert());
    let intersections = intersection(&coord1, &coord2);
    let min = intersections
        .iter()
        .map(|coord| {
            let a = coord1.iter().position(|c| c == coord).unwrap() + 1;
            let b = coord2.iter().position(|c| c == coord).unwrap() + 1;
            a + b
        })
        .min();
    println!("{}", min.unwrap());
}
