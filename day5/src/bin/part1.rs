fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let tape = day5::parse(&filename);
    let mut vm = day5::Vm::from(tape);

    while !vm.finished() {
        vm.cycle();
    }

    println!("tape: {:?}", vm.result());
}
