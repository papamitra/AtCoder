fn solve(s: &str, t: &str) -> String {
    let sn = s.len();
    let tn = t.len();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    #[derive(Clone)]
    enum Src {
        INV,
        UL,
        L,
        U,
    }

    let mut dp = vec![vec![(0, Src::INV); tn + 1]; sn + 1];

    for i in 0..sn {
        let sc = s[i];
        for j in 0..tn {
            let tc = t[j];
            if sc == tc {
                dp[i + 1][j + 1] = (dp[i][j].0 + 1, Src::UL);
            } else {
                if dp[i][j + 1].0 > dp[i + 1][j].0 {
                    dp[i + 1][j + 1] = (dp[i][j + 1].0, Src::U);
                } else {
                    dp[i + 1][j + 1] = (dp[i + 1][j].0, Src::L);
                }
            }
        }
    }

    let mut i = sn;
    let mut j = tn;
    let mut ans = "".to_owned();

    while i > 0 && j > 0 {
        match dp[i][j].1 {
            Src::UL => {
                ans.push(s[i - 1]);
                i -= 1;
                j -= 1;
            }
            Src::L => {
                j -= 1;
            }
            Src::U => {
                i -= 1;
            }
            Src::INV => unreachable!(),
        }
    }

    ans.chars().rev().collect()
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
