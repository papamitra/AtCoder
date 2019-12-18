fn solve(xys: Vec<(f64, f64)>) -> f64 {
    let n = xys.len();
    let mut nf = (n * (n - 1) / 2) as f64;

    let mut s = 0.0;
    for (i, &(xi, yi)) in xys.iter().enumerate() {
        if i == n - 1 {
            break;
        }
        for &(xj, yj) in xys[(i + 1)..].iter() {
            s += ((xj - xi).powf(2f64) + (yj - yi).powf(2f64)).sqrt();
        }
    }

    s / (nf as f64) * ((n - 1) as f64)
}

#[test]
fn solve_test() {
    //    assert_eq!(
    //        solve(vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]),
    //        2.2761423749
    //    );

    //    assert_eq!(solve(vec![(-879.0, 981.0), (-866.0, 890.0)]), 91.9238815543);

    /*    assert_eq!(
        solve(vec![
            (-406.0, 10.0),
            (512.0, 859.0),
            (494.0, 362.0),
            (-955.0, -475.0),
            (128.0, 553.0),
            (-986.0, -885.0),
            (763.0, 77.0),
            (449.0, 310.0)
        ]),
        7641.9817824387
    );*/
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
    let xys = (0..n).map(|_| parse_line!(f64, f64)).collect::<Vec<_>>();
    println!("{}", solve(xys));
}
