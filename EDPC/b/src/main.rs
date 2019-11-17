fn solve(k: i64, hs: Vec<i64>) -> i64 {
    use std::cmp::max;

    let n = hs.len();
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        let m = max(i as i64 - k, 0) as usize;
        for j in m..i {
            if dp[i] > dp[m] + (hs[j] - hs[i]).abs() {
                dp[i] = dp[m] + (hs[j] - hs[i]).abs();
            }
        }
    }

    dp[n - 1]
}

#[test]
fn solve_test() {
    assert_eq!(solve(3, vec![10, 30, 40, 50, 20]), 30);
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
    let (_n, k) = parse_line!(usize, i64);
    let hs = parse_vec!(i64);
    println!("{}", solve(k, hs));
}
