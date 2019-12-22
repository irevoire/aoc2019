use std::sync::mpsc::{Iter, Receiver, Sender};
use std::thread::JoinHandle;

pub struct VmCom {
    reader: Receiver<i64>,
    writer: Sender<i64>,
    vm: JoinHandle<crate::Vm>,
}

impl VmCom {
    pub fn new(reader: Receiver<i64>, writer: Sender<i64>, vm: JoinHandle<crate::Vm>) -> Self {
        Self { reader, writer, vm }
    }

    pub fn terminate(self) -> std::thread::Result<crate::Vm> {
        self.vm.join()
    }

    pub fn explode(self) -> (Receiver<i64>, Sender<i64>, JoinHandle<crate::Vm>) {
        (self.reader, self.writer, self.vm)
    }

    /// return true if the Vm is finished
    pub fn write(&mut self, i: i64) -> bool {
        match self.writer.send(i) {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    /// send a char to the intcode program
    /// return true if the Vm is finished
    pub fn putc(&mut self, c: char) -> bool {
        self.write(c as u8 as i64)
    }

    /// send a string
    /// return true if the Vm is finished
    pub fn puts(&mut self, s: &str) -> bool {
        s.chars().any(|c| self.putc(c))
    }

    /// send a line to the intcode program
    /// return true if the Vm is finished
    pub fn putl(&mut self, s: &str) -> bool {
        s.chars().chain("\n".chars()).any(|c| self.putc(c))
    }

    /// return None if the Vm is finished
    pub fn read(&mut self) -> Option<i64> {
        self.reader.recv().ok()
    }

    /// return an empty line if there is nothing to read
    pub fn read_line(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.getc() {
            s.push(c);
            if c == '\n' {
                break;
            }
        }
        s
    }

    /// read a char from intcode
    /// return None if the Vm is finished
    pub fn getc(&mut self) -> Option<char> {
        self.read().map(|i| i as u8 as char)
    }

    /// iterate over all the value the intcode can return
    pub fn read_iter(&mut self) -> Iter<i64> {
        self.reader.iter()
    }

    /// iterate over all the value the intcode can return and cast it as char
    pub fn getc_iter(&mut self) -> std::iter::Map<Iter<i64>, fn(i64) -> char> {
        self.reader.iter().map(|i| i as u8 as char)
    }
}
