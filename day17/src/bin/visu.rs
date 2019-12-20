use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let mut vm = run_from_file(&filename).unwrap();

    for c in vm.getc_iter() {
        print!("{}", c);
    }
}
