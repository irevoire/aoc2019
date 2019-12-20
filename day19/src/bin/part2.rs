use intcode::*;

fn get(tape: &Tape, x: i64, y: i64) -> i64 {
    let mut vm = run_from_tape(tape.clone());

    vm.write(x);
    vm.write(y);
    vm.read().unwrap()
}

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let tape = tape_from_file(&filename).unwrap();

    let mut start_of_beam = 0;
    for y in 4.. {
        for x in start_of_beam.. {
            match get(&tape, x, y) {
                1 => {
                    if get(&tape, x + 99, y) == 0 {
                        break; // go to the next line
                    } else if get(&tape, x, y + 99) == 1 {
                        println!("coord is {} {}", x, y);
                        println!("res is {}", x * 10000 + y);
                        return;
                    }
                }
                _ => start_of_beam = x,
            }
        }
    }
}
