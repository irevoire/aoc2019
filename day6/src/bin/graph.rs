fn main() {
    println!("digraph G {{");
    println!("\trankdir=\"LR\"");
    println!("\t\"gCOM\" [color=cyan]");
    println!("\t\"gYOU\" [color=green]");
    println!("\t\"gSAN\" [color=red]");

    let mut tree = day6::Tree::new();
    aoc::parser::lines().for_each(|l: String| {
        let r: Vec<&str> = l.split(')').collect();
        // we need to add a letter to the start of the node
        println!("\tg{} -> g{};", r[0], r[1]);
        tree.orbits(r[0], r[1]);
    });

    let mut liaison = Vec::new();
    tree.graph(&mut liaison);

    for l in liaison {
        if l.len() != 2 {
            continue;
        }
        println!("\tg{} -> g{} [color=red];", l[0], l[1]);
    }

    println!("}}");
}
