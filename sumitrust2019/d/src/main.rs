use std::collections::HashSet;

fn solve(s: &str) -> usize {
    let mut n = s.len();
    let s = s.chars().collect::<Vec<_>>();

    let mut h1 = HashSet::new();
    let mut v1 = Vec::new();

    for c in s.iter().rev() {
        h1.insert(c);
        v1.push(h1.len());
    }

    let v1 = v1.iter().rev().collect::<Vec<_>>();

    let mut h2 = HashSet::new();
    let mut ans = 0;
    let mut h3 = HashSet::new();

    for i in 0..(n - 2) {
        if !h3.contains(&s[i]) {
            for j in (i + 1)..(n - 1) {
                let k = (s[i], s[j]);

                if !h2.contains(&k) {
                    ans += *v1[j + 1];
                    h2.insert(k);
                }
            }
        }
        h3.insert(s[i]);
    }

    ans
}

#[test]
fn solve_test() {
    assert_eq!(solve("0224"), 3);
    assert_eq!(solve("123123"), 17);
    assert_eq!(solve("3141592653589793238"), 329);
}

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
        line.pop();
        line
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
        line.pop();
        line
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

fn main() {
    let _n = parse_line!(usize);
    let s = read_line!();
    println!("{}", solve(&s));
}
