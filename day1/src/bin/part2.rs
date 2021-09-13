fn fuel_computation(mut i: usize) -> usize {
    let mut total = 0;
    while i > 0 {
        i = (i / 3).saturating_sub(2);
        total += i;
    }
    total
}

fn main() {
    let requirement: usize = aoc::parser::lines_from_args_as::<usize>(1)
        .map(fuel_computation)
        .sum();
    println!("{}", requirement);
}
