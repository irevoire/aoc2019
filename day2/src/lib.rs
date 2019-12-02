pub mod tape;
pub mod vm;

pub use tape::Tape;
pub use vm::Vm;

use std::io;
use std::io::prelude::*;

pub fn parse() -> tape::Tape {
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .flat_map(|l| {
            l.unwrap()
                .split(',')
                .map(|el| el.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
