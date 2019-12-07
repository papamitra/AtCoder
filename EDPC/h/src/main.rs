static MOD: usize = 1000000007;

fn solve(aa: Vec<String>) -> usize {
    let h = aa.len();
    let w = aa[0].len();

    let mut dp = vec![0; w];
    dp[w - 1] = 1;

    for y in (0..h).rev() {
        let cs = aa[y].chars().collect::<Vec<_>>();

        if cs[w - 1] == '#' {
            dp[w - 1] = 0;
        }

        for x in (0..w - 1).rev() {
            if cs[x] == '#' {
                dp[x] = 0;
                continue;
            }

            dp[x] += dp[x + 1];
            dp[x] %= MOD;
        }
    }

    dp[0]
}

#[test]

fn solve_test() {
    assert_eq!(solve(vec!["..".to_owned(), "..".to_owned()]), 2);
    assert_eq!(solve(vec![".#".to_owned(), "#.".to_owned()]), 0);
    assert_eq!(
        solve(vec![
            "..#..".to_owned(),
            ".....".to_owned(),
            "#...#".to_owned(),
            ".....".to_owned(),
            "..#..".to_owned()
        ]),
        24
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
    let (h, _w) = parse_line!(usize, usize);

    let aa = (0..h).map(|_| read_line!()).collect::<Vec<_>>();

    println!("{}", solve(aa));
}
