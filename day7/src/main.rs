use itertools::Itertools;
use std::io::{Cursor, Read, Seek, SeekFrom};

fn execute_vm(memory: day7::Tape, input: &str, value: &str) -> String {
    let line = "\n";
    let reader: Cursor<Vec<u8>> = Cursor::new(
        input
            .bytes()
            .chain(line.bytes())
            .chain(value.bytes())
            .chain(line.bytes())
            .collect(),
    );
    let mut writer = Cursor::new(Vec::new());
    let mut vm = day7::Vm::from(memory, reader, &mut writer);

    while !vm.finished() {
        vm.cycle();
    }

    writer.seek(SeekFrom::Start(0)).unwrap();
    let mut res = String::new();
    writer.read_to_string(&mut res).unwrap();
    res
}

fn execute_chain(memory: day7::Tape, input: &[u8]) -> u32 {
    let mut input = input.iter();
    let res = execute_vm(memory.clone(), &input.next().unwrap().to_string(), "0"); // A
    let res = execute_vm(memory.clone(), &input.next().unwrap().to_string(), &res); // B
    let res = execute_vm(memory.clone(), &input.next().unwrap().to_string(), &res); // C
    let res = execute_vm(memory.clone(), &input.next().unwrap().to_string(), &res); // D
    let res = execute_vm(memory.clone(), &input.next().unwrap().to_string(), &res); // E
    res.trim().parse().unwrap()
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
