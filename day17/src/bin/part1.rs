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
    let mut map = vec![Vec::new()];

    for c in reader.iter() {
        let c = c as u8 as char;
        if c == '\n' {
            map.push(Vec::new());
        } else {
            map.last_mut().unwrap().push(c);
        }
    }
    map.pop(); // last elem is empty
    map.pop(); // two last elem are empty

    for line in map.iter() {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    let mut res = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if map[y][x] == '#'
                && map[y - 1][x] == '#'
                && map[y + 1][x] == '#'
                && map[y][x - 1] == '#'
                && map[y][x + 1] == '#'
            {
                map[y][x] = 'O';
                res += x * y;
            }
        }
    }

    println!("res is {}", res);
}
