use day12::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};

macro_rules! ppcm {
    ( $( $x:expr ),* ) => {
        {
            let mut res = 1;
            $(
                res = ppcm(res, $x);
            )*
            res
        }
    };
}

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let system: System = reader
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();

    let (x, y, z) = system.explode();
    let x = stabilize(x);
    println!("Stabilized dimension x at {}", x);
    let y = stabilize(y);
    println!("Stabilized dimension y at {}", y);
    let z = stabilize(z);
    println!("Stabilized dimension z at {}", z);

    println!("The system should stabilize at {}", ppcm!(x, y, z));
}

fn ppcm(mut a: usize, mut b: usize) -> usize {
    let c = a;
    let d = b;
    while a != b {
        if a > b {
            b += d;
        } else if a < b {
            a += c;
        }
    }
    a
}

fn stabilize(mut system: System) -> usize {
    let base = system.clone();

    let mut step = 1;
    system.update();

    while base != system {
        system.update();
        step += 1;
    }
    step
}
