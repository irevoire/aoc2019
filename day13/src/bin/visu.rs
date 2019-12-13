use day9::*;
use std::io::{stdout, Write};
use std::sync::mpsc::channel;
use termion::screen::AlternateScreen;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut tape = parse(&filename);
    tape[0] = 2; // insert quarters into the machine
    let (writer, vm_reader) = channel();
    let (vm_writer, reader) = channel();
    std::thread::spawn(move || {
        let mut vm = Vm::from(tape, vm_reader, vm_writer);
        while !vm.finished() {
            vm.cycle();
        }
    });

    let mut scores = 0;
    {
        // we are going to play in another screen
        let screen = AlternateScreen::from(stdout());
        let mut screen = termion::cursor::HideCursor::from(screen);
        let mut ball;
        let mut paddle = 0;
        loop {
            let x = reader.recv();
            if x.is_err() {
                break;
            }
            let x = x.unwrap();
            let y = reader.recv().unwrap();
            let id = reader.recv().unwrap();
            if x == -1 && y == 0 {
                scores = id;
                write!(screen, "{}score: {}\n", termion::cursor::Goto(30, 20), id).unwrap();
            } else {
                let tile = match id {
                    0 => ' ',
                    1 if x == 0 && y == 0 => '╔',
                    1 if x == 43 && y == 0 => '╗',
                    1 if x == 0 || x == 43 => '║',
                    1 => '═',
                    2 => '■',
                    3 => {
                        paddle = x;
                        '–'
                    }
                    4 => {
                        ball = x;
                        if paddle < ball {
                            writer.send(1).unwrap();
                        } else if paddle > ball {
                            writer.send(-1).unwrap();
                        } else {
                            writer.send(0).unwrap();
                        }
                        std::thread::sleep(std::time::Duration::from_millis(15));
                        'o'
                    }
                    id => panic!("got an unexpected tile id: '{}'", id),
                };

                write!(
                    screen,
                    "{}{}\n",
                    termion::cursor::Goto(x as u16 + 1, y as u16 + 1),
                    tile
                )
                .unwrap();
            }
        }
    }
    println!("final score is: {:?}", scores);
}
