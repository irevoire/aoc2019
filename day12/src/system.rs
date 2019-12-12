use crate::*;

#[derive(Clone)]
pub struct System {
    pub moons: Vec<Moon>,
    pub dimension: Dimension,
    update_vel: fn(&mut Moon, &Moon),
    update_pos: fn(&mut Moon),
}

impl PartialEq for System {
    fn eq(&self, system: &Self) -> bool {
        self.dimension == system.dimension
            && self
                .moons
                .iter()
                .zip(system.moons.iter())
                .all(|(a, b)| a == b)
    }
}

impl std::iter::FromIterator<Moon> for System {
    fn from_iter<I: IntoIterator<Item = Moon>>(iter: I) -> Self {
        let mut moons = Vec::new();
        for m in iter {
            moons.push(m);
        }
        System {
            moons,
            dimension: Dimension::All,
            update_vel: Moon::update_velocity,
            update_pos: Moon::update_position,
        }
    }
}

impl System {
    pub fn explode(&self) -> (Self, Self, Self) {
        (
            System {
                moons: self.moons.clone(),
                dimension: Dimension::X,
                update_vel: Moon::update_x_velocity,
                update_pos: Moon::update_x_position,
            },
            System {
                moons: self.moons.clone(),
                dimension: Dimension::Y,
                update_vel: Moon::update_y_velocity,
                update_pos: Moon::update_y_position,
            },
            System {
                moons: self.moons.clone(),
                dimension: Dimension::Z,
                update_vel: Moon::update_z_velocity,
                update_pos: Moon::update_z_position,
            },
        )
    }

    pub fn update_velocity(&mut self) {
        let update = self.update_vel;
        for _ in 0..self.moons.len() {
            self.moons.rotate_left(1);
            let (moon, others) = self.moons.split_first_mut().expect("Not enough moons");
            for m in others {
                (update)(moon, m);
            }
        }
    }

    pub fn update_position(&mut self) {
        let update = self.update_pos;
        self.moons.iter_mut().for_each(|m| {
            (update)(m);
        });
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
