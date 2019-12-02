pub struct Vm {
    pos: i32,
    mem: crate::tape::Tape,
}

impl Vm {
    pub fn new(a: i32, b: i32) -> Self {
        let mut mem = crate::parse();
        // restore gravity assist program
        mem[1] = a;
        mem[2] = b;

        Vm { pos: 0, mem }
    }

    pub fn from(mem: crate::tape::Tape) -> Self {
        Vm { pos: 0, mem }
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
            99 => return,
            op => panic!("Unknown opcode: {}", op),
        }
        self.pos += 4;
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
    }
}
