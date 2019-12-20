use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let tape = tape_from_file(&filename).unwrap();

    for y in 0..100 {
        for x in 0..100 {
            let mut vm = run_from_tape(tape.clone());

            vm.write(x);
            vm.write(y);
            if let Some(pulled) = vm.read() {
                match pulled {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => panic!("non mais wtf quoi"),
                }
            }
        }
        println!();
    }
}
