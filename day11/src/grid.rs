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

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all_black = true;
        for line in self.vec.iter() {
            for color in line {
                match color {
                    Color::Black => {
                        if all_black {
                            continue;
                        }
                        write!(f, " ")?;
                    }
                    Color::White => {
                        all_black = false;
                        write!(f, "█")?;
                    }
                };
            }
            if !all_black {
                // if the line is empty does nothing
                write!(f, "\n")?;
            }
            all_black = true; // skip the empty bits at the start
        }
        Ok(())
    }
}
