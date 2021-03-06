use day11::*;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut bot = Bot::from(&filename);
    let mut painted = std::collections::HashSet::new();
    while !bot.finished() {
        painted.insert(bot.coord());
        bot.cycle();
    }

    println!("{}", painted.len());
}
