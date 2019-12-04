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
    let mut groups = vec![Vec::new(); 1];
    for i in 1..s.len() {
        if s[i - 1] > s[i] {
            return false;
        }
        if s[i] == s[i - 1] {
            groups.last_mut().unwrap().push(s[i]);
        } else {
            groups.last_mut().unwrap().push(s[i - 1]);
            groups.push(Vec::new());
        }
    }
    if s[4] > s[5] {
        return false;
    } else {
        groups.last_mut().unwrap().push(s[5]);
    }

    groups.iter().any(|g| g.len() == 2)
}

fn main() {
    let mut args = std::env::args().skip(1); // Skiping the name of the binary
    let range = args.next().expect("give me range");
    let range: Vec<u32> = range.split('-').map(|e| e.parse().unwrap()).collect();
    let (start, end) = (range[0], range[1]);

    let mut count = 0;
    for i in start..=end {
        if valid(i) {
            count += 1;
        }
    }

    println!("{}", count);
}

#[test]
fn test() {
    assert_eq!(valid(111122), true);
    assert_eq!(valid(111233), true);
    assert_eq!(valid(123456), false);
    assert_eq!(valid(113456), true);
    assert_eq!(valid(111156), false);
}
