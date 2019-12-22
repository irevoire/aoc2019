#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Entrance,
    Empty,
    Wall,
    Key(char),
    Door(char),
}

pub use Cell::*;

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entrance => write!(f, "@"),
            Empty => write!(f, "."),
            Wall => write!(f, "#"),
            Key(c) => write!(f, "{}", c),
            Door(c) => write!(f, "{}", c.to_uppercase().next().unwrap()),
        }
    }
}

pub type Grid = Vec<Vec<Cell>>;

pub fn parse(s: &str) -> Grid {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => Entrance,
                    '.' => Empty,
                    '#' => Wall,
                    k @ 'a'..='z' => Key(k),
                    d @ 'A'..='Z' => Door(d.to_lowercase().next().unwrap()),
                    c => panic!("Unknown character: {:?}", c),
                })
                .collect::<Vec<Cell>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn from(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}
