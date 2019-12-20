use std::fs;

trait Res {
    fn res(&self) -> Self;
}

impl Res for i32 {
    fn res(&self) -> Self {
        let mut i = *self;
        if i < 0 {
            i *= -1;
        }
        i % 10
    }
}

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let mut input: Vec<i32> = fs::read_to_string(filename)
        .expect("Can’t open file")
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .expect("Can’t parse the provided input as a number") as i32
        })
        .collect();

    let base_phase = vec![0, 1, 0, -1];

    for _ in 0..100 {
        input = (0..input.len())
            .map(|i| {
                let phase = base_phase
                    .iter()
                    .flat_map(|el| std::iter::repeat(el).take(i + 1))
                    .cycle()
                    .skip(1);
                input
                    .iter()
                    .zip(phase.clone())
                    .map(|(input, phase)| input * phase)
                    .sum::<i32>()
                    .res()
            })
            .collect();
    }
    println!(
        "result is {}",
        input.iter().take(8).fold(0 as i32, |acc, el| acc * 10 + el)
    );
}
