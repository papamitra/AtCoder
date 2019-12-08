fn solve(s: &str) -> usize {
    let s = s.chars().collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - 1 - i] {
            ans += 1;
        }
    }

    ans
}

#[test]
fn solve_test() {
    assert_eq!(solve("redcoder"), 1);
    assert_eq!(solve("vvvvv"), 0);
    assert_eq!(solve("vvvvvv"), 0);
    assert_eq!(solve("abcabc"), 2);
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
    let s = read_line!();
    println!("{}", solve(&s));
}
