use crate::Moon;

pub struct System {
    pub moons: Vec<Moon>,
}

impl std::iter::FromIterator<Moon> for System {
    fn from_iter<I: IntoIterator<Item = Moon>>(iter: I) -> Self {
        let mut moons = Vec::new();
        for m in iter {
            moons.push(m);
        }
        System { moons }
    }
}

impl System {
    pub fn update_velocity(&mut self) {
        for _ in 0..self.moons.len() {
            self.moons.rotate_left(1);
            let (moon, others) = self.moons.split_first_mut().expect("Not enough moons");
            for m in others {
                moon.update_velocity(m);
            }
        }
    }

    pub fn update_position(&mut self) {
        for m in self.moons.iter_mut() {
            m.update_position();
        }
    }

    pub fn update(&mut self) {
        self.update_velocity();
        self.update_position();
    }

    pub fn energy(&self) -> i32 {
        self.moons.iter().map(|m| m.energy()).sum()
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "System {{")?;
        for m in self.moons.iter() {
            writeln!(f, "\t{}", m)?;
        }
        writeln!(f, "}}")
    }
}
