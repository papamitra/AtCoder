fn solve(abcs: Vec<(usize, usize, usize)>) -> usize {
    use std::cmp::max;

    let n = abcs.len();
    let mut dp = vec![vec![0; n + 1]; 3];

    for i in 1..(n + 1) {
        dp[0][i] = max(dp[1][i - 1], dp[2][i - 1]) + abcs[i - 1].0;
        dp[1][i] = max(dp[2][i - 1], dp[0][i - 1]) + abcs[i - 1].1;
        dp[2][i] = max(dp[0][i - 1], dp[1][i - 1]) + abcs[i - 1].2;
    }

    max(dp[0][n], max(dp[1][n], dp[2][n]))
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![(10, 40, 70), (20, 50, 80), (30, 60, 90)]), 210);
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
    let n = parse_line!(usize);
    let abcs = (0..n)
        .map(|_| parse_line!(usize, usize, usize))
        .collect::<Vec<_>>();
    println!("{}", solve(abcs));
}
