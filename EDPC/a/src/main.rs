fn solve(hs: Vec<i64>) -> i64 {
    use std::cmp::min;

    let n = hs.len();
    let mut dp = vec![0; n];

    dp[1] = (hs[1] - hs[0]).abs();

    for i in 2..n {
        dp[i] = min(
            dp[i - 2] + (hs[i] - hs[i - 2]).abs(),
            dp[i - 1] + (hs[i] - hs[i - 1]).abs(),
        );
    }

    dp[n - 1]
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![10, 30, 40, 20]), 30);
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
    let hs = parse_vec!(i64);
    println!("{}", solve(hs));
}
