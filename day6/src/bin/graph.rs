use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    println!("digraph G {{");
    println!("\trankdir=\"LR\"");
    reader.lines().for_each(|l| {
        let l = l.unwrap();
        let r: Vec<&str> = l.split(')').collect();
        // we need to add a letter to the start of the node
        println!("\tg{} -> g{};", r[0], r[1]);
    });
    println!("\t\"gCOM\" [style=filled, fillcolor=blue]");
    println!("\t\"gYOU\" [style=filled, fillcolor=green]");
    println!("\t\"gSAN\" [style=filled, fillcolor=red]");

    println!("}}");
}
