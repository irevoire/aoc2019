pub mod coord;
pub use coord::{Coord, Direction, Path};

pub fn intersection(a: &Vec<Coord>, b: &Vec<Coord>) -> Vec<Coord> {
    let mut res = Vec::new();
    for c in a {
        if b.contains(c) {
            res.push(c.clone());
        }
    }
    res
}
