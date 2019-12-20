use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut vm = run_from_file(&filename).unwrap();
    let mut blocks = 0;

    loop {
        match vm.read() {
            None => break,
            _ => (),
        };
        let _y = vm.read().unwrap();
        let id = vm.read().unwrap();
        match id {
            2 => blocks += 1,
            _ => (),
        }
    }

    println!("There is {} blocks on the screen at start", blocks);
}
