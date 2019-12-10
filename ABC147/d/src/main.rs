static MOD: u64 = 1000000007;

fn solve(aa: Vec<u64>) -> u64 {
    let n = aa.len();

    let mut ans = 0u64;

    for i in 0..61 {
        let mut cnt1 = 0;

        for a in aa.iter() {
            if ((a >> i) & 1u64) == 1u64 {
                cnt1 += 1;
            }
        }

        ans += (((cnt1 * (n - cnt1)) as u64) << i) % MOD;
        ans %= MOD;
    }

    ans
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![1, 2, 3]), 6);
    assert_eq!(solve(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]), 237);
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
    let aa = parse_vec!(u64);
    println!("{}", solve(aa));
}
