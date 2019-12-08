use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let mem: Vec<u8> = reader
        .lines()
        .flat_map(|l| {
            let l = l.unwrap();
            l.trim().bytes().map(|b| b - '0' as u8).collect::<Vec<u8>>()
        })
        .collect();
    let image = day8::Image::new(mem, 25, 6);
    let (mut layer, mut min) = (0, std::usize::MAX);
    for (idx, l) in image.layer().enumerate() {
        let zeros = l
            .iter()
            .fold(0, |acc, d| if *d == 0 { acc + 1 } else { acc });
        if zeros < min {
            layer = idx;
            min = zeros;
        }
    }

    let layer = image.layer_n(layer);
    let ones = layer
        .iter()
        .fold(0, |acc, d| if *d == 1 { acc + 1 } else { acc });
    let twos = layer
        .iter()
        .fold(0, |acc, d| if *d == 2 { acc + 1 } else { acc });
    println!("{}", ones * twos);
}
