fn solve(ps: Vec<usize>, k: usize) -> f64 {
    let n = ps.len();

    let mut sum = 0.0;
    let mut msum = 0.0;

    for i in 0..n {
        sum += (1.0 + ps[i] as f64) / 2.0;
        if i >= k {
            sum -= (1.0 + ps[i - k] as f64) / 2.0;
        }

        if msum < sum {
            msum = sum
        }
    }

    msum
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
    let (n, k) = parse_line!(usize, usize);
    let ps = parse_vec!(usize);

    println!("{}", solve(ps, k));
}
