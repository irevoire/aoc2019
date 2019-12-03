use crate::{Coord, Direction, Location, Path};

#[derive(Debug)]
pub struct Grid {
    vec: Vec<Vec<Location>>,
    coord: Coord,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            vec: Vec::new(),
            coord: Coord { x: 0, y: 0 },
        }
    }

    pub fn draw(&mut self, path: &Path) {
        for dir in path.vec.iter() {
            self.draw_dir(dir);
        }
    }

    fn draw_dir(&mut self, dir: &Direction) {
        if dir.value() == 0 {
            return;
        }
        self.coord += dir.one(); // move once in the right direction
        let current = self.coord;

        // update what was in the cell
        self[current] = match self[current] {
            Location::Empty => Location::Cable,
            Location::Cable => Location::Intersection,
            Location::Intersection => Location::Intersection,
        };

        self.draw_dir(&dir.dec());
    }
}

impl std::ops::Index<Coord> for Grid {
    type Output = Location;
    fn index(&self, i: crate::Coord) -> &Self::Output {
        if i.y >= self.vec.len() || i.x >= self.vec[i.y].len() {
            &crate::Location::Empty
        } else {
            &self.vec[i.y][i.x]
        }
    }
}

impl std::ops::IndexMut<Coord> for Grid {
    fn index_mut(&mut self, i: Coord) -> &mut Self::Output {
        let ylen = self.vec.len();
        if i.y >= ylen {
            let need = i.y - ylen + 1;
            self.vec.resize_with(ylen + need, || Vec::new());
        }
        let xlen = self.vec[i.y].len();
        if i.x >= xlen {
            let need = i.x - xlen + 1;
            self.vec[i.y].resize_with(xlen + need, || Location::Empty);
        }
        &mut self.vec[i.y][i.x]
    }
}
