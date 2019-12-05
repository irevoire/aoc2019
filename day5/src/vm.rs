use std::io;
use std::io::BufRead;

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

    pub fn cycle(&mut self) {
        let op = self.mem[self.pos];
        match op {
            1 => self.add(),
            2 => self.mul(),
            3 => self.input(),
            4 => self.output(),
            99 => return,
            op => panic!("Unknown opcode: {}", op),
        }
    }

    fn add(&mut self) {
        let pos_a = self.mem[self.pos + 1];
        let a = self.mem[pos_a];
        let pos_b = self.mem[self.pos + 2];
        let b = self.mem[pos_b];
        let pos_res = self.mem[self.pos + 3];

        if let Some(res) = a.checked_add(b) {
            self.mem[pos_res] = res;
        } else {
            self.mem[self.pos + 4] = 99;
        }

        self.pos += 4;
    }

    fn mul(&mut self) {
        let pos_a = self.mem[self.pos + 1];
        let a = self.mem[pos_a];
        let pos_b = self.mem[self.pos + 2];
        let b = self.mem[pos_b];
        let pos_res = self.mem[self.pos + 3];

        if let Some(res) = a.checked_mul(b) {
            self.mem[pos_res] = res;
        } else {
            self.mem[self.pos + 4] = 99;
        }

        self.pos += 4;
    }

    fn input(&mut self) {
        let pos_res = self.mem[self.pos + 1];
        println!("Waiting for an input: ");

        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .expect("Can't read on stdin");
        let input = line.trim().parse().expect("You must only provide integer");

        self.mem[pos_res] = input;

        self.pos += 2;
    }

    fn output(&mut self) {
        let pos_res = self.mem[self.pos + 1];
        println!("{}", self.mem[pos_res]);

        self.pos += 2;
    }
}
