use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut tape = tape_from_file(&filename).unwrap();
    tape[0] = 2; // insert quarters into the machine
    let mut vm = run_from_tape(tape);

    let mut score = 0;
    let mut ball;
    let mut paddle = 0;
    loop {
        let x = match vm.read() {
            None => break,
            Some(x) => x,
        };
        let y = vm.read().unwrap();
        let id = vm.read().unwrap();

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
                        vm.write(1);
                    } else if paddle > ball {
                        vm.write(-1);
                    } else {
                        vm.write(0);
                    }
                }
                _ => (),
            };
        }
    }
    println!("Score: {:?}", score);
}
