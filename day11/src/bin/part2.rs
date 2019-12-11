use day11::*;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let mut bot = Bot::from(&filename);
    let coord = bot.coord();
    bot.map[coord] = Color::White;

    while !bot.finished() {
        bot.cycle();
    }

    println!("{}", bot.map);
}
