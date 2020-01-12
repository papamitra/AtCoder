fn solve(k: usize, m: usize, aa: Vec<usize>) -> isize {
    let n = aa.len();
    let sum: usize = aa.iter().sum();
    let sum = sum as isize;

    let r = ((n + 1) * m) as isize - sum;

    if r < 0 {
        0
    } else if r > k as isize {
        -1
    } else {
        r
    }
}

#[test]
fn solve_test() {
    assert_eq!(solve(10, 7, vec![8, 10, 3, 6]), 8);
    assert_eq!(solve(100, 60, vec![100, 100, 100]), 0);
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
    let (_, k, m) = parse_line!(usize, usize, usize);
    let aa = parse_vec!(usize);

    println!("{}", solve(k, m, aa));
}
