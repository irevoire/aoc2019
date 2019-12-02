fn main() {
    let mut vm = day2::Vm::new();

    while !vm.finished() {
        vm.cycle();
    }

    println!("tape: {:?}", vm.result());
}
