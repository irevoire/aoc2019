use day9::*;
use std::sync::mpsc::channel;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut tape = parse(&filename);
    tape[0] = 2; // insert quarters into the machine
    let (writer, vm_reader) = channel();
    let (vm_writer, reader) = channel();
    std::thread::spawn(move || {
        let mut vm = Vm::from(tape, vm_reader, vm_writer);
        while !vm.finished() {
            vm.cycle();
        }
    });

    let mut score = 0;
    let mut ball;
    let mut paddle = 0;
    loop {
        let x = reader.recv();
        if x.is_err() {
            break;
        }
        let x = x.unwrap();
        let y = reader.recv().unwrap();
        let id = reader.recv().unwrap();
        if x == -1 && y == 0 {
            score = id;
        } else {
            match id {
                3 => {
                    paddle = x;
                }
                4 => {
                    ball = x;
                    if paddle < ball {
                        writer.send(1).unwrap();
                    } else if paddle > ball {
                        writer.send(-1).unwrap();
                    } else {
                        writer.send(0).unwrap();
                    }
                }
                _ => (),
            };
        }
    }
    println!("Score: {:?}", score);
}
