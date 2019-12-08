#![allow(non_snake_case)]
use itertools::Itertools;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

fn execute_vm(memory: day7::Tape, reader: Receiver<i32>, writer: Sender<i32>) {
    let mut vm = day7::Vm::from(memory, reader, writer);

    while !vm.finished() {
        vm.cycle();
    }
}

fn execute_chain(memory: day7::Tape, input: &[u8]) -> i32 {
    let mut input = input.iter();
    let (sender, receiverA) = channel();
    sender.send(*input.next().unwrap() as i32).unwrap();
    sender.send(0).unwrap();
    let (senderA, receiverB) = channel();
    senderA.send(*input.next().unwrap() as i32).unwrap();
    let (senderB, receiverC) = channel();
    senderB.send(*input.next().unwrap() as i32).unwrap();
    let (senderC, receiverD) = channel();
    senderC.send(*input.next().unwrap() as i32).unwrap();
    let (senderD, receiverE) = channel();
    senderD.send(*input.next().unwrap() as i32).unwrap();
    let (senderE, result) = channel();

    execute_vm(memory.clone(), receiverA, senderA); // A
    execute_vm(memory.clone(), receiverB, senderB); // B
    execute_vm(memory.clone(), receiverC, senderC); // C
    execute_vm(memory.clone(), receiverD, senderD); // D
    execute_vm(memory.clone(), receiverE, senderE); // E

    result.recv().unwrap()
}

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let tape = day7::parse(&filename);
    let res = (0..=4)
        .permutations(5)
        .map(|perm| execute_chain(tape.clone(), &perm))
        .max()
        .unwrap();
    println!("max is: {}", res);
}
