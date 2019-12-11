#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}
use Direction::*;

impl std::ops::Add<Direction> for Direction {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match (self, dir) {
            (Left, Right) | (Right, Left) => Up,
            (Left, Left) | (Right, Right) => Down,
            (Left, d) | (Right, d) => panic!("unhandled direction {:?}", d),
            (Up, Right) | (Down, Left) => Right,
            (Up, Left) | (Down, Right) => Left,
            (Up, d) | (Down, d) => panic!("unhandled direction {:?}", d),
        }
    }
}

impl std::ops::AddAssign<Direction> for Direction {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}

impl From<Direction> for i64 {
    fn from(dir: Direction) -> i64 {
        match dir {
            Direction::Left => 0,
            Direction::Right => 1,
            d => panic!("Unhandled conversion of direction {:?} into integer", d),
        }
    }
}
impl From<i64> for Direction {
    fn from(i: i64) -> Direction {
        match i {
            0 => Direction::Left,
            1 => Direction::Right,
            i => panic!("Unhandled conversion of {} into direction", i),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new() -> Self {
        Coord { x: 100, y: 100 }
    }

    pub fn from(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

impl std::ops::Add<Direction> for Coord {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match dir {
            Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl std::ops::AddAssign<Direction> for Coord {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}
