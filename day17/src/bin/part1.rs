use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let mut vm = run_from_file(&filename).unwrap();

    let mut map = vec![Vec::new()];

    for c in vm.getc_iter() {
        if c == '\n' {
            map.push(Vec::new());
        } else {
            map.last_mut().unwrap().push(c);
        }
    }
    map.pop(); // last elem is empty
    map.pop(); // two last elem are empty

    let mut res = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if map[y][x] == '#'
                && map[y - 1][x] == '#'
                && map[y + 1][x] == '#'
                && map[y][x - 1] == '#'
                && map[y][x + 1] == '#'
            {
                map[y][x] = 'O';
                res += x * y;
            }
        }
    }

    for line in map.iter() {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    println!("res is {}", res);
}
