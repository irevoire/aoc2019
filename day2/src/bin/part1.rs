fn main() {
    let mut vm = day2::Vm::new(12, 2);

    while !vm.finished() {
        vm.cycle();
    }

    println!("tape: {:?}", vm.result());
}
