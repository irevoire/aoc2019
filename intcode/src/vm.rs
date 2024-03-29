use crossbeam_channel::{unbounded, Receiver, Sender};

pub enum Mode {
    Position,
    Immediate,
    Relative,
}

pub use Mode::*;

pub struct Vm {
    pos: i64,
    relative_base: i64,
    mem: crate::tape::Tape,
    reader: Option<Receiver<i64>>,
    writer: Option<Sender<i64>>,
}

impl Vm {
    /// create a VM from a memory tape you should provide,
    /// a Reader and a Writer
    pub fn from(mem: crate::Tape, reader: Receiver<i64>, writer: Sender<i64>) -> Self {
        Vm {
            pos: 0,
            relative_base: 0,
            mem,
            reader: Some(reader),
            writer: Some(writer),
        }
    }

    pub fn run_from(tape: crate::Tape) -> crate::VmCom {
        let (writer, vm_reader) = unbounded();
        let (vm_writer, reader) = unbounded();
        let vm = Vm::from(tape, vm_reader, vm_writer);
        let vm = vm.run();
        crate::VmCom::new(reader, writer, vm)
    }

    pub fn run(mut self) -> std::thread::JoinHandle<Self> {
        std::thread::spawn(move || {
            while !self.finished() {
                self.cycle();
            }
            self.reader = None;
            self.writer = None;
            self
        })
    }

    /// return true if the execution of the VM is finished
    pub fn finished(&self) -> bool {
        self.mem[self.pos] == 99
    }

    fn opcode(&self) -> (Mode, Mode, Mode, u8) {
        let mut opcode = self.mem[self.pos];

        let op = (opcode % 100) as u8;
        opcode /= 100;

        let c = match opcode % 10 {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            mode => panic!("Unknown mode: {}", mode),
        };
        opcode /= 10;

        let b = match opcode % 10 {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            mode => panic!("Unknown mode: {}", mode),
        };
        opcode /= 10;

        let a = match opcode % 10 {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            mode => panic!("Unknown mode: {}", mode),
        };

        (a, b, c, op)
    }

    fn get(&self, pos: i64, mode: Mode) -> i64 {
        match mode {
            Position => self.mem[self.mem[pos]],
            Immediate => self.mem[pos],
            Relative => {
                let pos = self.mem[pos];
                self.mem[self.relative_base + pos]
            }
        }
    }

    fn get_mut(&mut self, pos: i64, mode: Mode) -> &mut i64 {
        match mode {
            Position => {
                let pos = self.mem[pos];
                &mut self.mem[pos]
            }
            Immediate => &mut self.mem[pos],
            Relative => {
                let base = self.relative_base;
                let pos = self.mem[pos];
                &mut self.mem[base + pos]
            }
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
            // relative base
            9 => self.relative_base(),
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
            self.pos += 4;
        } else {
            eprintln!("an overflow was encountered");
            self.mem[self.pos] = 99;
        }
    }

    fn mul(&mut self) {
        let (a, b, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);
        let b = self.get(self.pos + 2, b);
        let a = self.get_mut(self.pos + 3, a);

        if let Some(res) = c.checked_mul(b) {
            *a = res;
            self.pos += 4;
        } else {
            eprintln!("an overflow was encountered");
            self.mem[self.pos] = 99;
        }
    }

    fn input(&mut self) {
        let input = match &self.reader {
            Some(reader) => reader.recv().expect("Can't read input"),
            None => return,
        };

        let (_, _, c, _) = self.opcode();
        let c = self.get_mut(self.pos + 1, c);

        *c = input;

        self.pos += 2;
    }

    fn output(&mut self) {
        let (_, _, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);

        if let Some(writer) = &self.writer {
            writer
                .send(c)
                // if a channel was closed finish all threads
                .unwrap_or_else(|_| self.mem[self.pos + 2] = 99);
        } else {
            return;
        }

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

    fn relative_base(&mut self) {
        let (_, _, c, _) = self.opcode();
        let c = self.get(self.pos + 1, c);

        self.relative_base += c;

        self.pos += 2;
    }
}
