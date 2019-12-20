use intcode::*;
use std::io::{stdout, Write};
use termion::screen::AlternateScreen;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut tape = tape_from_file(&filename).unwrap();
    tape[0] = 2; // insert quarters into the machine
    let mut vm = run_from_tape(tape);

    let mut scores = 0;
    {
        // we are going to play in another screen
        let screen = AlternateScreen::from(stdout());
        let mut screen = termion::cursor::HideCursor::from(screen);
        let mut ball;
        let mut paddle = 0;
        loop {
            let x = match vm.read() {
                Some(x) => x,
                None => break,
            };
            let y = vm.read().unwrap();
            let id = vm.read().unwrap();
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
                            vm.write(1);
                        } else if paddle > ball {
                            vm.write(-1);
                        } else {
                            vm.write(0);
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
