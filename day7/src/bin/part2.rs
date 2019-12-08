#![allow(non_snake_case)]
use itertools::Itertools;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

fn execute_vm(memory: day7::Tape, reader: Receiver<i32>, writer: Sender<i32>) -> i32 {
    let mut vm = day7::Vm::from(memory, reader, writer);

    while !vm.finished() {
        vm.cycle();
    }

    vm.result()
}

fn execute_chain(memory: day7::Tape, input: &[u8]) -> i32 {
    let mut input = input.iter();
    let (senderE, receiverA) = channel();
    senderE.send(*input.next().unwrap() as i32).unwrap();
    senderE.send(0).unwrap();
    let (senderA, receiverB) = channel();
    senderA.send(*input.next().unwrap() as i32).unwrap();
    let (senderB, receiverC) = channel();
    senderB.send(*input.next().unwrap() as i32).unwrap();
    let (senderC, receiverD) = channel();
    senderC.send(*input.next().unwrap() as i32).unwrap();
    let (senderD, receiverE) = channel();
    senderD.send(*input.next().unwrap() as i32).unwrap();

    let tmemory = memory.clone();
    std::thread::spawn(move || {
        execute_vm(tmemory, receiverA, senderA); // A
    });
    let tmemory = memory.clone();
    std::thread::spawn(move || {
        execute_vm(tmemory.clone(), receiverB, senderB); // B
    });
    let tmemory = memory.clone();
    std::thread::spawn(move || {
        execute_vm(tmemory.clone(), receiverC, senderC); // C
    });
    let tmemory = memory.clone();
    std::thread::spawn(move || {
        execute_vm(tmemory.clone(), receiverD, senderD); // D
    });
    let tmemory = memory.clone();
    let t = std::thread::spawn(move || {
        execute_vm(tmemory.clone(), receiverE, senderE) // E
    });
    t.join().unwrap()
}

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let tape = day7::parse(&filename);
    let res = (5..=9)
        .permutations(5)
        .map(|perm| execute_chain(tape.clone(), &perm))
        .max()
        .unwrap();
    println!("max is: {}", res);
}
