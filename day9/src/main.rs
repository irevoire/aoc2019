use std::sync::mpsc::channel;

fn main() {
    let mut args = std::env::args().skip(1); // skip the name of the binary
    let filename = args.next().expect("give me the path to your program");
    let intcode_input = args.next().expect("Give the input of the intcode program");

    let (input, reader) = channel();
    let (writer, res) = channel();
    input
        .send(
            intcode_input
                .parse()
                .expect("your input is not a valid number"),
        )
        .unwrap();

    std::thread::spawn(move || {
        let tape = day9::parse(&filename);
        let mut vm = day9::Vm::from(tape, reader, writer);

        while !vm.finished() {
            vm.cycle();
        }
        // once the thread finish the channel will be closed
    });

    for r in res {
        println!("{}", r);
    }
}
