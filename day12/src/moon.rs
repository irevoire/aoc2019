#[derive(PartialEq, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl std::str::FromStr for Coord {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .split(',')
            .map(|s| s.trim_matches(|s| s == 'x' || s == 'y' || s == 'z' || s == '=' || s == ' '))
            .collect();

        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        let z = coords[2].parse::<i32>()?;

        Ok(Coord { x, y, z })
    }
}

impl Coord {
    pub fn new() -> Self {
        Coord { x: 0, y: 0, z: 0 }
    }

    pub fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(PartialEq, Clone)]
pub struct Moon {
    position: Coord,
    velocity: Coord,
}

impl std::str::FromStr for Moon {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let position = s.trim_matches(|s| s == '<' || s == '>').parse()?;

        Ok(Moon {
            position,
            velocity: Coord::new(),
        })
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x={:3}, y={:3}, z={:3}", self.x, self.y, self.z)
    }
}

impl Moon {
    pub fn update_velocity(&mut self, moon: &Self) {
        // a > b return true or false which are translated into 1 or 0
        // and since a > b and a < b can’t be true at the same time one of
        // the two following statement will return 0 and the other 1
        self.velocity.x += (self.position.x < moon.position.x) as i32;
        self.velocity.x -= (self.position.x > moon.position.x) as i32;

        self.velocity.y += (self.position.y < moon.position.y) as i32;
        self.velocity.y -= (self.position.y > moon.position.y) as i32;

        self.velocity.z += (self.position.z < moon.position.z) as i32;
        self.velocity.z -= (self.position.z > moon.position.z) as i32;
    }

    pub fn update_x_velocity(&mut self, moon: &Self) {
        self.velocity.x += (self.position.x < moon.position.x) as i32;
        self.velocity.x -= (self.position.x > moon.position.x) as i32;
    }
    pub fn update_y_velocity(&mut self, moon: &Self) {
        self.velocity.y += (self.position.y < moon.position.y) as i32;
        self.velocity.y -= (self.position.y > moon.position.y) as i32;
    }

    pub fn update_z_velocity(&mut self, moon: &Self) {
        self.velocity.z += (self.position.z < moon.position.z) as i32;
        self.velocity.z -= (self.position.z > moon.position.z) as i32;
    }

    pub fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    pub fn update_x_position(&mut self) {
        self.position.x += self.velocity.x;
    }

    pub fn update_y_position(&mut self) {
        self.position.y += self.velocity.y;
    }

    pub fn update_z_position(&mut self) {
        self.position.z += self.velocity.z;
    }

    pub fn energy(&self) -> i32 {
        self.position.energy() * self.velocity.energy()
    }
}

impl std::fmt::Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos=<{}>, vel=<{}>", self.position, self.velocity)
    }
}
