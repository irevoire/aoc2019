use day18::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let file = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path of your input");

    let grid = parse(&std::fs::read_to_string(file).unwrap());

    let mut arr = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Key(c) = grid[y][x] {
                arr.insert(c, distances(&grid, &Coord::from(x, y)));
            } else if grid[y][x] == Entrance {
                arr.insert('@', distances(&grid, &Coord::from(x, y)));
            }
        }
    }

    println!("{:?}", solver(arr));
}

fn solver(data: HashMap<char, HashMap<char, (usize, HashSet<char>)>>) -> usize {
    _solver('@', &HashSet::new(), data)
}

fn _solver(
    start: char,
    keys: &HashSet<char>,
    mut data: HashMap<char, HashMap<char, (usize, HashSet<char>)>>,
) -> usize {
    if data.len() == 1 {
        return 0; // finished
    }
    let current = data.remove(&start).expect("Non mais wtf quoi");
    current
        .iter()
        .filter_map(|(key, (dist, doors))| {
            // if you already got all the needed keys
            if keys.is_superset(doors) && data.contains_key(key) {
                let mut keys = keys.clone();
                keys.insert(*key);
                Some(dist + _solver(*key, &keys, data.clone()))
            } else {
                None
            }
        })
        .min()
        .expect("Mais wtf quoi !")
}

/// return a hashmap containing for every point: the distance to go to
/// it and the list of door on the path
fn distances(grid: &Grid, c: &Coord) -> HashMap<char, (usize, HashSet<char>)> {
    let mut hash = HashMap::new();
    _distances(grid, *c, 0, &mut HashSet::new(), &HashSet::new(), &mut hash);
    if let Key(c) = grid[c.y][c.x] {
        hash.remove(&c);
    }
    hash
}

/// positions: a vector containing all the positions where you already went
/// keys: a vector containing all teh keys you still need to get
fn _distances(
    grid: &Grid,
    coord: Coord,
    depth: usize,
    positions: &mut HashSet<Coord>,
    doors: &HashSet<char>,
    keys: &mut HashMap<char, (usize, HashSet<char>)>,
) {
    if positions.contains(&coord) {
        return;
    }

    positions.insert(coord);

    let mut doors = doors.clone();
    match grid[coord.y][coord.x] {
        Empty => (),    // ok
        Wall => return, // stop
        Door(c) => {
            doors.insert(c);
        }
        Entrance => {
            // keys.insert('@', (depth, doors.clone()));
            ();
        }
        Key(c) => {
            keys.insert(c, (depth, doors.clone()));
        }
    }

    _distances(
        grid,
        Coord::from(coord.x + 1, coord.y),
        depth + 1,
        positions,
        &doors,
        keys,
    );
    _distances(
        grid,
        Coord::from(coord.x - 1, coord.y),
        depth + 1,
        positions,
        &doors,
        keys,
    );
    _distances(
        grid,
        Coord::from(coord.x, coord.y + 1),
        depth + 1,
        positions,
        &doors,
        keys,
    );
    _distances(
        grid,
        Coord::from(coord.x, coord.y - 1),
        depth + 1,
        positions,
        &doors,
        keys,
    );
}
