#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Left(usize),
    Up(usize),
    Down(usize),
    Right(usize),
}

impl Direction {
    pub fn value(&self) -> usize {
        match self {
            Direction::Left(n) => *n,
            Direction::Up(n) => *n,
            Direction::Down(n) => *n,
            Direction::Right(n) => *n,
        }
    }

    pub fn dec(&self) -> Direction {
        match self {
            Direction::Left(n) => Direction::Left(n - 1),
            Direction::Up(n) => Direction::Up(n - 1),
            Direction::Down(n) => Direction::Down(n - 1),
            Direction::Right(n) => Direction::Right(n - 1),
        }
    }

    pub fn one(&self) -> Direction {
        match self {
            Direction::Left(_) => Direction::Left(1),
            Direction::Up(_) => Direction::Up(1),
            Direction::Down(_) => Direction::Down(1),
            Direction::Right(_) => Direction::Right(1),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("R", n) => Ok(Direction::Right(n.parse()?)),
            ("U", n) => Ok(Direction::Up(n.parse()?)),
            ("L", n) => Ok(Direction::Left(n.parse()?)),
            ("D", n) => Ok(Direction::Down(n.parse()?)),
            _ => panic!("invalid input"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Path {
    pub vec: Vec<Direction>,
}

impl Path {
    pub fn convert(&self) -> Vec<Coord> {
        let mut base = Coord { x: 0, y: 0 };
        self.vec
            .iter()
            .flat_map(|dir| std::iter::repeat(dir.one()).take(dir.value()))
            .map(|dir| {
                base += dir;
                base.clone()
            })
            .collect()
    }
}

impl std::str::FromStr for Path {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split(',')
            .map(|d| d.parse::<Direction>())
            .collect::<Result<Vec<Direction>, _>>()?;
        Ok(Path { vec })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn start(&self) -> bool {
        self.x == self.y && self.x == 0
    }
}

/// we are defining addition between a coordinate and a direction
impl std::ops::Add<Direction> for Coord {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match dir {
            Direction::Left(a) => Coord {
                x: self.x - a as i32,
                y: self.y,
            },
            Direction::Down(a) => Coord {
                x: self.x,
                y: self.y - a as i32,
            },
            Direction::Up(a) => Coord {
                x: self.x,
                y: self.y + a as i32,
            },
            Direction::Right(a) => Coord {
                x: self.x + a as i32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_from_str() {
        assert_eq!(Ok(Direction::Up(5)), "U5".parse());
        assert_eq!(Ok(Direction::Down(78)), "D78".parse());
        assert_eq!(Ok(Direction::Right(4)), "R004".parse());

        assert_eq!(Ok(vec![Direction::Right(4)]), "R004".parse());
    }

    #[test]
    fn vec_direction_from_str() {
        assert_eq!(Ok(Direction::Up(5)), "U5".parse());
        assert_eq!(Ok(Direction::Down(78)), "D78".parse());
        assert_eq!(Ok(Direction::Right(4)), "R004".parse());
    }
}
