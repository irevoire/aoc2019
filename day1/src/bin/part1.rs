fn main() {
    let requirement: i32 = aoc::parser::lines::<i32>()
        .map(|i| i / 3 - 2)
        .sum();
    println!("{}", requirement);
}

