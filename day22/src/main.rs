use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let f = File::open(filename).expect("can’t open your file");
    let f = BufReader::new(f);

    let mut deck = day22::Deck::new(10007);

    for line in f.lines() {
        deck.apply(&line.unwrap());
    }

    println!("{}", deck.vec.iter().position(|c| *c == 2019).unwrap());
}
