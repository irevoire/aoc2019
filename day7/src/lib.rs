pub mod tape;
pub mod vm;

pub use tape::Tape;
pub use vm::Vm;

use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn parse(file: &str) -> tape::Tape {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .flat_map(|l| {
            l.unwrap()
                .split(',')
                .map(|el| el.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
