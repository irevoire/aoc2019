use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let mut vm = run_from_file(&filename).unwrap();

    vm.putl("OR A T");
    vm.putl("AND B T");
    vm.putl("AND C T");
    vm.putl("NOT T T");
    vm.putl("OR E J");
    vm.putl("OR H J");
    vm.putl("AND T J");
    vm.putl("AND D J");
    vm.putl("RUN");

    vm.putl("RUN");

    let mut last_value = 0;
    for c in vm.read_iter() {
        last_value = c;
    }
    println!("res: {}", last_value);
}
