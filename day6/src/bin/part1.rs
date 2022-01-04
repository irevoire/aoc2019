fn main() {
    let mut tree = day6::Tree::new();
    aoc::parser::lines().for_each(|l: String| {
        let r: Vec<&str> = l.split(')').collect();
        tree.orbits(r[0], r[1]);
    });

    aoc::answer!(
        "The total number of direct and indirect orbits is {}.",
        tree.total_orbits()
    );
}
