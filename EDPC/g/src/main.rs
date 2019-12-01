use std::cmp::max;
use std::collections::HashMap;

fn solve(xys: Vec<(usize, usize)>, n: usize) -> usize {
    let mut m = vec![vec![]; n + 1];

    for (x, y) in xys {
        m[x].push(y);
    }

    let mut dp = vec![None; n + 1];

    fn f(x: usize, dp: &mut Vec<Option<usize>>, m: &Vec<Vec<usize>>) -> usize {
        if let Some(n) = dp[x] {
            n
        } else {
            if m[x].is_empty() {
                dp[x] = Some(0);
                0
            } else {
                let mut maxp = 0;
                for &y in m[x].iter() {
                    maxp = max(maxp, f(y, dp, m) + 1);
                }
                dp[x] = Some(maxp);
                maxp
            }
        }
    }

    let mut ans = 0;
    for x in 1..(n + 1) {
        ans = max(ans, f(x, &mut dp, &m));
    }

    ans
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![(1, 2), (1, 3), (3, 2), (2, 4), (3, 4)], 4), 3);
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
    let (n, m) = parse_line!(usize, usize);

    let xys = (0..m)
        .map(|_| parse_line!(usize, usize))
        .collect::<Vec<_>>();
    println!("{}", solve(xys, n));
}
