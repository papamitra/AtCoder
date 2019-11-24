fn solve(k: i64, hs: Vec<i64>) -> i64 {
    use std::cmp::max;

    let n = hs.len();
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        let m = max(i as i64 - k, 0) as usize;
        for j in m..i {
            if dp[i] > dp[j] + (hs[i] - hs[j]).abs() {
                dp[i] = dp[j] + (hs[i] - hs[j]).abs();
            }
        }
    }

    dp[n - 1]
}

#[test]
fn solve_test() {
    assert_eq!(solve(3, vec![10, 30, 40, 50, 20]), 30);
    assert_eq!(solve(1, vec![10, 20, 10]), 20);
    assert_eq!(solve(100, vec![10, 10]), 0);
    assert_eq!(solve(4, vec![40, 10, 20, 70, 80, 10, 20, 70, 80, 60]), 40);
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
