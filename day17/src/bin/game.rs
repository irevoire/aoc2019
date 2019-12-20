use intcode::*;

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let mut tape = tape_from_file(&filename).unwrap();
    tape[0] = 2;
    let mut vm = run_from_tape(tape);

    let main_routine = "A,B,A,C,B,C,B,A,C,B\n";
    let func_a = "L,6,R,8,R,12,L,6,L,8\n";
    let func_b = "L,10,L,8,R,12\n";
    let func_c = "L,8,L,10,L,6,L,6\n";
    let realtime = "n\n";

    let mut size = 0;
    let mut map: Vec<Vec<char>> = vec![Vec::new()];
    for c in vm.getc_iter() {
        if c == '\n' {
            if map.last().unwrap().is_empty() {
                break;
            }
            size += 1;
            map.push(Vec::new());
        } else {
            map.last_mut().unwrap().push(c);
        }
        print!("{}", c);
    }
    map.pop(); // remove the last useless line

    let mut c = path(&map);
    print!("The path is: ");

    while !c.is_empty() {
        let func_a = func_a.trim();
        let func_b = func_b.trim();
        let func_c = func_c.trim();

        if c.starts_with(func_a) {
            print!("\x1B[32m{}\x1B[m", func_a);
            c = c[func_a.len()..].to_string();
        } else if c.starts_with(func_b) {
            print!("\x1B[31m{}\x1B[m", func_b);
            c = c[func_b.len()..].to_string();
        } else if c.starts_with(func_c) {
            print!("\x1B[34m{}\x1B[m", func_c);
            c = c[func_c.len()..].to_string();
        } else {
            let idx_a = c.find(func_a).unwrap_or(c.len());
            let idx_b = c.find(func_b).unwrap_or(c.len());
            let idx_c = c.find(func_c).unwrap_or(c.len());
            let min = *[idx_a, idx_b, idx_c].iter().min().unwrap();
            print!("{}", &c[..min]);
            c = c[min..].to_string();
        }
        if c.starts_with(",") {
            print!(",");
            c = c[1..].to_string();
        }
    }
    println!();
    print!("The constructed path is: ");
    for c in main_routine.chars() {
        let func_a = func_a.trim();
        let func_b = func_b.trim();
        let func_c = func_c.trim();
        match c {
            'A' => print!("\x1B[32m{}\x1B[m", func_a),
            'B' => print!("\x1B[31m{}\x1B[m", func_b),
            'C' => print!("\x1B[34m{}\x1B[m", func_c),
            ',' => print!("|"),
            _ => (),
        }
    }
    println!();

    print!("{} {}", vm.read_line().trim(), main_routine);
    vm.puts(main_routine);

    print!("\x1B[32m{}\x1B[m {}", vm.read_line().trim(), func_a);
    vm.puts(func_a);

    print!("\x1B[31m{}\x1B[m {}", vm.read_line().trim(), func_b);
    vm.puts(func_b);

    print!("\x1B[34m{}\x1B[m {}", vm.read_line().trim(), func_c);
    vm.puts(func_c);

    print!("{} {}", vm.read_line().trim(), realtime);
    vm.puts(realtime);

    print_map(&mut vm);
    let i = vm.read().unwrap();
    println!("res: {}", i);
    return;

    let mut first = true;
    loop {
        if first {
            first = false;
        } else {
            print!("\x1B[{}A", size);
        }
        std::thread::sleep_ms(50);
        size = print_map(&mut vm);
        if size == 0 {
            break;
        }
    }
}

fn print_map(vm: &mut VmCom) -> usize {
    let mut size = 0;
    let mut last = ' ';
    for c in vm.getc_iter() {
        if c == '\n' {
            if last == '\n' {
                break;
            }
            size += 1;
        }
        last = c;
        print!("{}", c);
    }
    size
}

fn path(map: &Vec<Vec<char>>) -> String {
    let mut res = String::new();
    let mut pos = (0, 0);
    let mut dir = ' ';
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' || map[y][x] == '>' || map[y][x] == '<' || map[y][x] == 'v' {
                pos = (x, y);
                dir = map[y][x];
            }
        }
    }

    let old_dir = dir;
    // find in which direction you need to go to be on the track
    match (
        map[pos.1][pos.0 - 1],
        map[pos.1][pos.0 + 1],
        map[pos.1 - 1][pos.0],
        map[pos.1 + 1][pos.0],
    ) {
        ('#', _, _, _) => dir = '<',
        (_, '#', _, _) => dir = '>',
        (_, _, '#', _) => dir = '^',
        (_, _, _, '#') => dir = 'v',
        _ => panic!("nowhere to start"),
    }
    match (old_dir, dir) {
        ('>', '^') | ('^', '<') | ('<', 'v') | ('v', '>') => res.push_str("L,"),
        ('>', 'v') | ('v', '<') | ('<', '^') | ('^', '>') => res.push_str("R,"),
        ('>', '<') | ('<', '>') | ('^', 'v') | ('v', '^') => res.push_str("L,L,"),
        _ => (),
    }
    loop {
        // find how much time you can go in the direction
        let mut i = 0;
        match dir {
            '>' => {
                while pos.0 != map[pos.1].len() - 1 && map[pos.1][pos.0 + 1] == '#' {
                    i += 1;
                    pos.0 += 1;
                }
            }
            '<' => {
                while pos.0 != 0 && map[pos.1][pos.0 - 1] == '#' {
                    i += 1;
                    pos.0 -= 1;
                }
            }
            '^' => {
                while pos.1 != 0 && map[pos.1 - 1][pos.0] == '#' {
                    i += 1;
                    pos.1 -= 1;
                }
            }
            'v' => {
                while pos.1 != map.len() - 1 && map[pos.1 + 1][pos.0] == '#' {
                    i += 1;
                    pos.1 += 1;
                }
            }
            _ => panic!("wtf"),
        }
        res.push_str(&format!("{},", i));

        // find the next direction
        let old_dir = dir;
        // find in which direction you need to go to be on the track
        match (
            dir,
            pos.0.checked_sub(1).map(|x| map[pos.1][x]),
            map[pos.1].get(pos.0 + 1),
            pos.1.checked_sub(1).map(|y| map[y][pos.0]),
            map.get(pos.1 + 1).map(|a| a[pos.0]),
        ) {
            ('^', Some('#'), _, _, _) => dir = '<',
            ('^', _, Some('#'), _, _) => dir = '>',
            ('v', Some('#'), _, _, _) => dir = '<',
            ('v', _, Some('#'), _, _) => dir = '>',
            ('<', _, _, Some('#'), _) => dir = '^',
            ('<', _, _, _, Some('#')) => dir = 'v',
            ('>', _, _, Some('#'), _) => dir = '^',
            ('>', _, _, _, Some('#')) => dir = 'v',

            _ => break, // finished
        }
        match (old_dir, dir) {
            ('>', '^') | ('^', '<') | ('<', 'v') | ('v', '>') => res.push_str("L,"),
            ('>', 'v') | ('v', '<') | ('<', '^') | ('^', '>') => res.push_str("R,"),
            _ => panic!("strange things are happenning"),
        }
    }

    res
}
