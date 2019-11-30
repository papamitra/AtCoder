fn solve(wvs: Vec<(usize, usize)>, maxw: usize) -> usize {
    use std::cmp::max;
    let n = wvs.len();

    // dp[i][j]: j番目までの品物から重さの総和がi以下となるように選んだときの価値の総和の最大
    let mut dp = vec![vec![0; n + 1]; maxw + 1];

    for i in 1..(maxw + 1) {
        for j in 1..(n + 1) {
            let (w, v) = wvs[j - 1];
            if w > i {
                dp[i][j] = dp[i][j - 1];
            } else {
                dp[i][j] = max(dp[i][j - 1], dp[i - w][j - 1] + v);
            }
        }
    }

    dp[maxw][n]
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![(3, 30), (4, 50), (5, 60)], 8), 90);
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
    let (n, w) = parse_line!(usize, usize);
    let wvs = (0..n)
        .map(|_| parse_line!(usize, usize))
        .collect::<Vec<_>>();
    println!("{}", solve(wvs, w));
}
