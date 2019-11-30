fn solve(s: &str, t: &str) -> String {
    let sn = s.len();
    let tn = t.len();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![(0, "".to_owned()); tn + 1]; sn + 1];

    for i in 0..sn {
        let sc = s[i];
        for j in 0..tn {
            let tc = t[j];
            if sc == tc {
                let mut news = dp[i][j].1.clone();
                news.push(sc);
                dp[i + 1][j + 1] = (dp[i][j].0 + 1, news);
            } else {
                if dp[i][j + 1].0 > dp[i + 1][j].0 {
                    dp[i + 1][j + 1] = dp[i][j + 1].clone();
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].clone();
                }
            }
        }
    }

    dp[sn][tn].1.clone()
}

#[test]
fn solve_test() {
    assert_eq!(solve("axyb", "abyxb"), "ayb".to_owned());
    assert_eq!(solve("aa", "xayaz"), "aa".to_owned());
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
    let t = read_line!();

    println!("{}", solve(&s, &t));
}
