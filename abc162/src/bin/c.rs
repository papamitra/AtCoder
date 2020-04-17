#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.trim_right().to_string()
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn gcd(a: usize, b: usize) -> usize {
    use std::cmp::{max, min};
    let mi = min(a, b);
    let ma = max(a, b);

    let r = ma % mi;
    if r == 0 {
        return mi;
    }

    gcd(mi, r)
}

fn main() {
    let k = parse_line!(usize);

    let mut sum = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                sum += gcd(gcd(a, b), c);
            }
        }
    }

    println!("{}", sum);
}
