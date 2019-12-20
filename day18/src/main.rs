use day18::*;

fn main() {
    let file = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path of your input");

    let grid = parse(&std::fs::read_to_string(file).unwrap());

    let res = res(&grid);
    println!("res is: {}", res);
}

fn res(grid: &Grid) -> u32 {
    let mut keys = Vec::new();
    let mut c = Coord::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Key(c) = grid[y][x] {
                keys.push(c);
            } else if grid[y][x] == Entrance {
                c = Coord::from(x, y);
            }
        }
    }
    _res(grid, c, &Vec::new(), &keys)
}

/// positions: a vector containing all the positions where you already went
/// keys: a vector containing all teh keys you still need to get
fn _res(grid: &Grid, coord: Coord, positions: &[Coord], keys: &[char]) -> u32 {
    if positions.contains(&coord) {
        return 1000;
    }
    match grid[coord.y][coord.x] {
        Entrance | Empty => (),              // ok
        Wall => return 1000,                 // stop
        Door(c) if !keys.contains(&c) => (), // ok
        Door(_c) => return 1000,             // you can’t pass the door without the key
        Key(c) if !keys.contains(&c) => (),  // ok you already got this key
        Key(c) => {
            // finished
            if keys.len() == 1 {
                return 0;
            }
            // restart everything from the new position
            let mut keys = keys.to_vec();
            let pos = keys.iter().position(|x| *x == c).unwrap();
            keys.remove(pos);
            return _res(grid, coord, &Vec::new(), &keys);
        }
    }
    let mut positions = positions.to_vec();
    positions.push(coord);
    let min = vec![
        _res(grid, Coord::from(coord.x + 1, coord.y), &positions, keys),
        _res(grid, Coord::from(coord.x - 1, coord.y), &positions, keys),
        _res(grid, Coord::from(coord.x, coord.y + 1), &positions, keys),
        _res(grid, Coord::from(coord.x, coord.y - 1), &positions, keys),
    ];
    let min = min.iter().min();

    1 + min.unwrap()
}
