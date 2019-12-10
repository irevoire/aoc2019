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

use std::f64::consts::PI;

#[allow(illegal_floating_point_literal_pattern)]
fn dist(a: &Coord, b: &Coord) -> f64 {
    let x = (b.x - a.x) as f64;
    let y = (b.y - a.y) as f64;
    let y = -y;
    match (x, y) {
        (0., 0.) => 0.,
        (0., y) if y > 0. => 0.,
        (x, 0.) if x >= 0. => PI / 2.,
        (x, 0.) if x < 0. => 3. * PI / 2.,
        (x, y) if y < 0. => (x / y).atan() + PI,
        (x, y) if x < 0. && y > 0. => (x / y).atan() + 2. * PI,
        (x, y) => (x / y).atan(),
    }
}

fn all_visible(from: Coord, asteroids: &[Coord]) -> Vec<usize> {
    let mut res = Vec::new();
    for (idx, c) in asteroids.iter().enumerate() {
        let hide = line(from, *c);
        if !asteroids.iter().any(|from| hide(*from)) {
            res.push(idx);
        }
    }
    res
}

fn main() {
    let mut args = std::env::args().skip(1); // Skiping the name of the binary
    let filename = args.next().expect("give me the path to your program");
    let station: Vec<i32> = args
        .next()
        .expect("give me the coordinates of your station")
        .split(',')
        .map(|x| x.parse::<i32>().expect("malformed coordinates"))
        .collect();
    let station = coord(station[0], station[1]);

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

    let mut vaporized = 0;

    while vaporized < 200 {
        let mut visible = all_visible(station, &asteroids);
        if visible.len() + vaporized < 200 {
            vaporized += visible.len();
            visible.sort_unstable();
            // we remove from the end to be sure we don’t need to adjust the indexes
            visible.iter().rev().for_each(|i| {
                asteroids.remove(*i);
            });
            continue;
        }
        visible.sort_unstable_by(|a, b| {
            dist(&station, &asteroids[*a])
                .partial_cmp(&dist(&station, &asteroids[*b]))
                .unwrap()
        });
        let coord = asteroids[visible[200 - vaporized]];
        println!("{}", coord.x * 100 + coord.y);
        return;
    }
}

#[test]
fn test() {
    let hide = line(coord(11, 13), coord(11, -1000));
    assert!(hide(coord(11, 12)));
}

#[test]
fn test_atan() {
    let base = coord(11, 13);
    panic!(
        "first {} second {}",
        dist(&base, &coord(11, 12)) * 180.0 / 3.14,
        dist(&base, &coord(11, 14)) * 180.0 / 3.14,
    );
}
