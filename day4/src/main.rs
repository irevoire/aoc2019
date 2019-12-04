fn split(mut n: u32) -> Vec<u8> {
    let mut res = Vec::new();
    while n > 0 {
        res.push((n % 10) as u8);
        n /= 10;
    }
    res.reverse();
    res
}

fn valid(n: u32) -> bool {
    let s = split(n);
    if s.len() != 6 {
        return false;
    }
    let mut adjacent = false;
    s.windows(2).all(|v| {
        let (a, b) = (v[0], v[1]);
        if a == b {
            adjacent = true;
        }
        if a > b {
            false
        } else {
            true
        }
    }) && adjacent
}

fn main() {
    let mut args = std::env::args().skip(1); // Skiping the name of the binary
    let range = args.next().expect("give me range");
    let range: Vec<u32> = range.split('-').map(|e| e.parse().unwrap()).collect();
    let (start, end) = (range[0], range[1]);

    let mut count = 0;
    for i in start..end {
        if valid(i) {
            count += 1;
        }
    }
    println!(
        "between {} and {} theres is {} valid number",
        start, end, count
    );
}
