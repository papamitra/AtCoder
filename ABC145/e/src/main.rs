fn solve(t: usize, abs: Vec<(usize, usize)>) -> usize {
    use std::cmp::max;

    let mut abs = abs;
    abs.sort_by_key(|x| x.0);

    let mut dp = vec![0; t + 3001];

    for &(a, b) in abs.iter() {
        for i in (0..t).rev() {
            dp[i + a] = max(dp[i] + b, dp[i + a]);
        }
    }

    dp.into_iter().max().unwrap()
}

#[test]
fn solve_test() {
    assert_eq!(solve(60, vec![(10, 10), (10, 20), (10, 30)]), 60);
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
    let (n, t) = parse_line!(usize, usize);
    let abs = (0..n)
        .map(|_| parse_line!(usize, usize))
        .collect::<Vec<_>>();
    println!("{}", solve(t, abs));
}
