use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

fn coord(x: i32, y: i32) -> Coord {
    Coord { x, y }
}

fn line(a: Coord, b: Coord) -> Box<dyn Fn(Coord) -> bool> {
    if b.x == a.x {
        // vertical line
        let max = b.y.max(a.y);
        let min = b.y.min(a.y);
        Box::new(move |c: Coord| c.x == a.x && c.y < max && c.y > min)
    } else if b.y == a.y {
        // horizontal line
        let max = b.x.max(a.x);
        let min = b.x.min(a.x);
        Box::new(move |c: Coord| c.y == a.y && c.x < max && c.x > min)
    } else {
        // not aligned
        let x = b.y - a.y;
        let x = -x;
        let y = b.x - a.x;
        let value = x * a.x + y * a.y;

        let max_x = b.x.max(a.x);
        let min_x = b.x.min(a.x);
        let max_y = b.y.max(a.y);
        let min_y = b.y.min(a.y);
        Box::new(move |c: Coord| {
            ((x * c.x + y * c.y) == value)
                && c.x < max_x
                && c.x > min_x
                && c.y < max_y
                && c.y > min_y
        })
    }
}

fn see(a: Coord, asteroids: &[Coord]) -> u32 {
    let mut res = 0;
    for c in asteroids {
        let hide = line(a, *c);
        if !asteroids.iter().any(|a| hide(*a)) {
            res += 1;
        }
    }
    res
}

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    let mut asteroids: Vec<Coord> = reader
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let l = line.unwrap();
            l.trim()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some(coord(x as i32, y as i32)),
                    _ => None,
                })
                .collect::<Vec<Coord>>()
        })
        .collect();

    let mut max = 0;
    let mut best = coord(0, 0);
    for _ in 0..asteroids.len() {
        asteroids.rotate_left(1);
        let (a, others) = asteroids.split_first().expect("You provided empty lines");

        let tmp = see(*a, others);

        if tmp > max {
            max = tmp;
            best = *a;
        }
    }

    println!("{:?} {}", best, max);
}

#[test]
fn test() {
    let hide = line(coord(1, 0), coord(3, 4));
    assert!(hide(coord(2, 2)));
    let hide = line(coord(3, 4), coord(1, 0));
    assert!(hide(coord(2, 2)));
}
