use std::fs::File;
use std::io::{prelude::*, BufReader};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

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
    let image = day8::Image::new(mem, WIDTH, HEIGHT);
    let mut res = [2; WIDTH * HEIGHT];
    for l in image.layer() {
        for i in 0..l.len() {
            if res[i] == 2 {
                res[i] = l[i];
            }
        }
    }

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            let p = res[h * WIDTH + w];
            if p == 1 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
