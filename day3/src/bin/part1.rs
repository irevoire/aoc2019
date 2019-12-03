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
    let intersections = intersection(&paths[0].convert(), &paths[1].convert());
    let min = intersections.iter().map(|c| c.x.abs() + c.y.abs()).min();
    println!("Nearest intersection is at: {}", min.unwrap());
}
