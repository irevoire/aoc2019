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
    let cost = reaction.cost("FUEL");
    println!("1 FUEL cost {} OREs", cost);
    println!(
        "you can generate {} FUEL with 1000000000000 OREs",
        1000000000000. / cost
    );
}
