fn compute(mut mem: day2::Tape, a: u8, b: u8) -> i32 {
    assert!(a < 100 && b < 100);
    mem[1] = a as i32;
    mem[2] = b as i32;

    let mut vm = day2::Vm::from(mem);

    while !vm.finished() {
        vm.cycle();
    }

    vm.result()
}

fn main() {
    let tape = day2::parse();

    for a in 0..100 {
        for b in 0..100 {
            let res = compute(tape.clone(), a, b);
            if res == 19690720 {
                println!(
                    "The solution is {}, with a: {} and b: {}",
                    a as u32 * 100 + b as u32,
                    a,
                    b
                );
                return;
            }
        }
    }
}
