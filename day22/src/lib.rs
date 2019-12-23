#[derive(Debug)]
pub struct Deck {
    pub vec: Vec<i32>,
}

impl Deck {
    pub fn new(size: usize) -> Self {
        Deck {
            vec: (0 as i32..size as i32).collect(),
        }
    }

    /// parse and apply a line
    pub fn apply(&mut self, line: &str) {
        if line.starts_with("deal into new stack") {
            self.new_stack();
        } else if line.starts_with("cut ") {
            self.cut(line.trim().split(' ').last().unwrap().parse().unwrap());
        } else if line.starts_with("deal with increment ") {
            self.increment(line.trim().split(' ').last().unwrap().parse().unwrap());
        } else {
            println!("Fucked up with {}", line);
        }
    }

    pub fn new_stack(&mut self) {
        self.vec.reverse();
    }

    pub fn cut(&mut self, idx: i32) {
        let (a, b) = match idx.is_negative() {
            true => self.vec.split_at(self.vec.len() - idx.abs() as usize),
            false => self.vec.split_at(idx as usize),
        };
        self.vec = b.iter().chain(a).map(|a| *a).collect();
    }

    pub fn increment(&mut self, increment: usize) {
        let mut vec = vec![0; self.vec.len()];
        self.new_stack();
        vec[0] = self.vec.pop().expect("wtf quoi");
        let mut idx = increment;
        while idx != 0 {
            vec[idx] = self.vec.pop().expect("non mais wtf");
            idx += increment;
            idx %= vec.len();
        }
        self.vec = vec;
    }
}
