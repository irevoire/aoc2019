fn main() {
    let requirement: i32 = aoc::parser::lines_from_args_as::<i32>(1)
        .map(|i| i / 3 - 2)
        .sum();
    println!("{}", requirement);
}

