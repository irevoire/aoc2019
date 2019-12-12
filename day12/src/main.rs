use day12::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let mut system: System = reader
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();

    for _ in 0..1000 {
        system.update();
    }
    println!("energy: {}", system.energy());
}
