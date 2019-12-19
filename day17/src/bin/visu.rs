use day9::*;
use std::sync::mpsc::channel;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let tape = parse(&filename);
    let (_, vm_reader) = channel();
    let (vm_writer, reader) = channel();
    std::thread::spawn(move || {
        let mut vm = Vm::from(tape, vm_reader, vm_writer);
        while !vm.finished() {
            vm.cycle();
        }
    });

    for c in reader.iter() {
        print!("{}", c as u8 as char);
    }
}
