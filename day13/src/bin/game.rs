use intcode::*;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut tape = tape_from_file(&filename).unwrap();
    tape[0] = 2; // insert quarters into the machine
    let (reader, writer, _vm) = run_from_tape(tape).explode();

    std::thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Esc) => break,
                Event::Key(Key::Ctrl('c')) => break,
                Event::Key(Key::Left) => writer.send(-1).unwrap(),
                Event::Key(Key::Right) => writer.send(1).unwrap(),
                Event::Key(Key::Char(' ')) => writer.send(0).unwrap(),
                _ => (),
            }
        }
    });

    let mut scores = Vec::new();
    {
        let stdout = stdout().into_raw_mode().unwrap();
        // we are going to play in another screen
        let screen = AlternateScreen::from(stdout);
        let mut screen = termion::cursor::HideCursor::from(screen);
        loop {
            let x = reader.recv();
            if x.is_err() {
                break;
            }
            let x = x.unwrap();
            let y = reader.recv().unwrap();
            let id = reader.recv().unwrap();
            if x == -1 && y == 0 {
                scores.push(id);
                write!(screen, "{}score: {}\n", termion::cursor::Goto(30, 20), id).unwrap();
            } else {
                let tile = match id {
                    0 => ' ',
                    1 if x == 0 && y == 0 => '╔',
                    1 if x == 43 && y == 0 => '╗',
                    1 if x == 0 || x == 43 => '║',
                    1 => '═',
                    2 => '■',
                    3 => '–',
                    4 => 'o',
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

    println!("all scores: {:?}", scores);
    println!(
        "all increments: {:?}",
        scores
            .windows(2)
            .map(|arr| arr[1] - arr[0])
            .collect::<Vec<i64>>()
    );
    println!("final score is: {:?}", scores.last().unwrap());
}
