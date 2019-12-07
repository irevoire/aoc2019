fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let tape = day7::parse(&filename);
    let mut vm = day7::Vm::from(tape.clone(), std::io::stdin());

    while !vm.finished() {
        vm.cycle();
    }
}
