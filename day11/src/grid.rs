use crate::Color;

#[derive(Debug)]
pub struct Grid {
    vec: Vec<Vec<Color>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid { vec: Vec::new() }
    }
}

impl std::ops::Index<crate::Coord> for Grid {
    type Output = Color;
    fn index(&self, i: crate::Coord) -> &Self::Output {
        if i.y >= self.vec.len() || i.x >= self.vec[i.y].len() {
            &Color::Black
        } else {
            &self.vec[i.y][i.x]
        }
    }
}

impl std::ops::IndexMut<crate::Coord> for Grid {
    fn index_mut(&mut self, i: crate::Coord) -> &mut Self::Output {
        if i.y >= self.vec.len() {
            self.vec.resize(i.y + 1, Vec::new());
        }
        let line_length = self.vec[i.y].len();
        if i.x >= line_length {
            self.vec[i.y].resize(i.x + 1, Color::Black);
        }
        &mut self.vec[i.y][i.x]
    }
}
