const MOD: i64 = 1000000007;

fn solve(aa: Vec<i64>) -> i64 {
    let mut ans: i64 = 1;
    let mut cs = vec![0i64, 0, 0];

    for i in 0..aa.len() {
        ans *= cs.iter().filter(|&c| *c == aa[i]).count() as i64;
        ans %= MOD;
        for j in 0..3 {
            if cs[j] == aa[i] {
                cs[j] += 1;
                break;
            }
        }
    }

    ans
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![0, 1, 2, 3, 4, 5]), 3);
    assert_eq!(solve(vec![0, 0, 0]), 6);
    assert_eq!(
        solve(vec![
            0, 0, 1, 0, 1, 2, 1, 2, 3, 2, 3, 3, 4, 4, 5, 4, 6, 5, 7, 8, 5, 6, 6, 7, 7, 8, 8, 9, 9,
            10, 10, 11, 9, 12, 10, 13, 14, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 15, 16, 16, 16,
            17, 17, 17
        ]),
        115295190
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
    let _n = parse_line!(usize);
    let aa = parse_vec!(i64);
    println!("{}", solve(aa));
}
