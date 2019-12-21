use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let mut vm = run_from_file(&filename).unwrap();

    vm.putl("NOT A J");
    vm.putl("NOT B T");
    vm.putl("OR T J");
    vm.putl("NOT C T");
    vm.putl("OR T J");
    vm.putl("AND D J");
    vm.putl("WALK");

    let mut last_value = 0;
    for c in vm.read_iter() {
        last_value = c;
    }
    println!("res: {}", last_value);
}
