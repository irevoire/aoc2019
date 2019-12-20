pub mod deplacement;
mod grid;

pub use deplacement::{Coord, Direction};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl From<Color> for i64 {
    fn from(color: Color) -> i64 {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl From<i64> for Color {
    fn from(i: i64) -> Color {
        match i {
            0 => Color::Black,
            1 => Color::White,
            c => panic!("Unhandled conversion of {} into color", c),
        }
    }
}

pub struct Bot {
    finished: bool,
    direction: Direction,
    position: Coord,
    pub map: grid::Grid,
    brain: intcode::VmCom,
}

impl Bot {
    pub fn from(file: &str) -> Self {
        Bot {
            finished: false,
            direction: Direction::Up,
            position: Coord::new(),
            map: grid::Grid::new(),
            brain: intcode::run_from_file(file).unwrap(),
        }
    }

    pub fn cycle(&mut self) {
        let color = self.map[self.position];
        if self.brain.write(color.into()) {
            self.finished = true;
            return;
        }
        match self.brain.read() {
            Some(color) => self.map[self.position] = color.into(),
            _ => {
                self.finished = true;
                return;
            }
        }
        match self.brain.read() {
            Some(direction) => self.direction += direction.into(),
            _ => {
                self.finished = true;
                return;
            }
        }

        self.position += self.direction;
    }

    pub fn coord(&self) -> Coord {
        self.position
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}
