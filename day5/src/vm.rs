use std::io;
use std::io::BufRead;

pub enum Mode {
    Position,
    Immediate,
}

pub use Mode::*;

pub struct Vm {
    pos: i32,
    mem: crate::tape::Tape,
    reader: io::BufReader<io::Stdin>,
}

impl Vm {
    pub fn from(mem: crate::tape::Tape) -> Self {
        let stdin = io::stdin();
        let reader = io::BufReader::new(stdin);
        Vm {
            pos: 0,
            mem,
            reader,
        }
    }

    pub fn finished(&self) -> bool {
        self.mem[self.pos] == 99
    }

    pub fn result(&self) -> i32 {
        self.mem[0]
    }

    fn opcode(&self) -> (Mode, Mode, Mode, u8) {
        let mut opcode = self.mem[self.pos];

        let op = (opcode % 100) as u8;
        opcode /= 100;

        let c = match opcode % 10 {
            0 => Position,
            1 => Immediate,
            mode => panic!("Unknown mode: {}", mode),
        };
        opcode /= 10;

        let b = match opcode % 10 {
            0 => Position,
            1 => Immediate,
            mode => panic!("Unknown mode: {}", mode),
        };
        opcode /= 10;

        let a = match opcode % 10 {
            0 => Position,
            1 => Immediate,
            mode => panic!("Unknown mode: {}", mode),
        };

        (a, b, c, op)
    }

    fn get(&self, pos: i32, mode: Mode) -> i32 {
        match mode {
            Position => self.mem[self.mem[pos]],
            Immediate => self.mem[pos],
        }
    }

    fn get_mut(&mut self, pos: i32, mode: Mode) -> &mut i32 {
        match mode {
            Position => {
                let pos = self.mem[pos];
                &mut self.mem[pos]
            }
            Immediate => &mut self.mem[pos],
        }
    }

    pub fn cycle(&mut self) {
        let op = self.mem[self.pos] % 100;
        match op {
            // operation
            1 => self.add(),
            2 => self.mul(),
            // io
            3 => self.input(),
            4 => self.output(),
            // jump
            5 => self.if_true(),
            6 => self.if_false(),
            // condition
            7 => self.less_than(),
            8 => self.equals(),
            99 => return,
            op => panic!("Unknown opcode: {}", op),
        }
    }

    fn add(&mut self) {
        let (a, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);
        let a = self.get_mut(self.pos + 3, a);

        if let Some(res) = c.checked_add(b) {
            *a = res;
        } else {
            *a = 99;
        }

        self.pos += 4;
    }

    fn mul(&mut self) {
        let (a, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);
        let a = self.get_mut(self.pos + 3, a);

        if let Some(res) = c.checked_mul(b) {
            *a = res;
        } else {
            *a = 99;
        }

        self.pos += 4;
    }

    fn input(&mut self) {
        println!("Waiting for an input: ");

        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .expect("Can't read on stdin");
        let input = line.trim().parse().expect("You must only provide integer");

        let (_, _, c, _) = self.opcode();
        let c = self.get_mut(self.pos + 1, c);

        *c = input;

        self.pos += 2;
    }

    fn output(&mut self) {
        let (_, _, c, _) = self.opcode();
        let c = self.get_mut(self.pos + 1, c);

        println!("{}", *c);

        self.pos += 2;
    }

    fn if_true(&mut self) {
        let (_, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);

        if c != 0 {
            self.pos = b;
        } else {
            self.pos += 3;
        }
    }

    fn if_false(&mut self) {
        let (_, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);

        if c == 0 {
            self.pos = b;
        } else {
            self.pos += 3;
        }
    }

    fn less_than(&mut self) {
        let (a, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);
        let a = self.get_mut(self.pos + 3, a);

        if c < b {
            *a = 1;
        } else {
            *a = 0;
        }

        self.pos += 4;
    }

    fn equals(&mut self) {
        let (a, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);
        let a = self.get_mut(self.pos + 3, a);

        if c == b {
            *a = 1;
        } else {
            *a = 0;
        }

        self.pos += 4;
    }
}
