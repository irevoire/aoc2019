use day9::*;
use std::sync::mpsc::channel;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let tape = parse(&filename);
    let (_, vm_reader) = channel();
    let (vm_writer, reader) = channel();
    std::thread::spawn(move || {
        let mut vm = Vm::from(tape, vm_reader, vm_writer);
        while !vm.finished() {
            vm.cycle();
        }
    });
    let mut blocks = 0;

    loop {
        let x = reader.recv();
        if x.is_err() {
            break;
        }
        let _y = reader.recv().unwrap();
        let id = reader.recv().unwrap();
        match id {
            2 => blocks += 1,
            _ => (),
        }
    }

    println!("There is {} blocks on the screen at start", blocks);
}
