use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let mut tree = day6::Tree::new();
    reader.lines().for_each(|l| {
        let l = l.unwrap();
        let r: Vec<&str> = l.split(')').collect();
        tree.orbits(r[0], r[1]);
    });

    println!("{}", tree.total_orbits());
}
