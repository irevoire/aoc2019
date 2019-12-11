pub mod deplacement;
mod grid;

pub use deplacement::{Coord, Direction};
use std::sync::mpsc::{channel, Receiver, Sender};

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
    map: grid::Grid,
    reader: Receiver<i64>,
    writer: Sender<i64>,
}

impl Bot {
    pub fn from(from: &str) -> Self {
        let tape = day9::parse(from);
        let (writer, vm_reader) = channel();
        let (vm_writer, reader) = channel();
        std::thread::spawn(move || {
            let mut brain = day9::Vm::from(tape, vm_reader, vm_writer);
            while !brain.finished() {
                brain.cycle();
            }
        });

        Bot {
            finished: false,
            direction: Direction::Up,
            position: Coord::new(),
            map: grid::Grid::new(),
            reader,
            writer,
        }
    }

    pub fn cycle(&mut self) {
        let color = self.map[self.position];
        if let Err(_) = self.writer.send(color.into()) {
            self.finished = true;
            return;
        }
        if let Ok(color) = self.reader.recv() {
            self.map[self.position] = color.into();
        } else {
            self.finished = true;
            return;
        }
        if let Ok(direction) = self.reader.recv() {
            self.direction += direction.into();
        } else {
            self.finished = true;
            return;
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
