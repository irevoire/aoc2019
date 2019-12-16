use day14::Reaction;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut f = String::new();
    let mut file = File::open(filename).expect("Can’t open file");
    file.read_to_string(&mut f).unwrap();

    let mut reaction: Reaction = f.parse().unwrap();
    reaction.generate(1, String::from("FUEL"));
    println!("used ORE: {}", reaction.used("ORE"));
}
