fn solve(wvs: Vec<(usize, usize)>, maxw: usize) -> usize {
    use std::cmp::min;
    let n = wvs.len();

    let mut dp = vec![vec![std::usize::MAX; n + 1]; 100001];

    for i in 0..dp[0].len() {
        dp[0][i] = 0;
    }

    for i in 1..dp.len() {
        for j in 1..dp[0].len() {
            let (w, v) = wvs[j - 1];

            if (v > i || dp[i - v][j - 1] == std::usize::MAX) {
                dp[i][j] = dp[i][j - 1];
            } else {
                dp[i][j] = min(dp[i][j - 1], dp[i - v][j - 1] + w);
            }
        }
    }

    for i in (1..dp.len()).rev() {
        let mut minw = std::usize::MAX;
        for j in 1..dp[0].len() {
            if dp[i][j] < minw {
                minw = dp[i][j];
            }
        }

        if minw <= maxw {
            return i;
        }
    }

    return 0;
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![(3, 30), (4, 50), (5, 60)], 8), 90);
    assert_eq!(
        solve(vec![(6, 5), (5, 6), (6, 4), (6, 6), (3, 5), (7, 2)], 15),
        17
    );
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
