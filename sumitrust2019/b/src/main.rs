fn solve(n: usize) -> Option<usize> {
    let n: f64 = n as f64;

    let ans = (n / 1.08).ceil();

    if n as usize != (ans * 1.08) as usize {
        None
    } else {
        Some(ans as usize)
    }
}

#[test]
fn solve_test() {
    assert_eq!(solve(10), Some(10));
    assert_eq!(solve(432), Some(400));
    assert_eq!(solve(1079), None);
    assert_eq!(solve(1001), Some(927));
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

    let ans = solve(n);
    println!(
        "{}",
        if let Some(ans) = ans {
            format!("{}", ans)
        } else {
            ":(".to_owned()
        }
    );
}
