fn solve(ps: Vec<(usize, String)>) -> (usize, usize) {
    let mut ans = vec![false; 100001];
    let mut was = vec![0; 100001];

    let mut wa = 0;
    let mut ac = 0;
    for (p, r) in ps {
        if ans[p] {
            continue;
        }

        if r == "WA" {
            was[p] += 1;
        } else {
            ans[p] = true;
            ac += 1;
            wa += was[p];
        }
    }

    (ac, wa)
}

#[test]
fn solve_test() {
    assert_eq!(
        solve(vec![
            (1, "WA".to_owned()),
            (1, "AC".to_owned()),
            (2, "WA".to_owned()),
            (2, "AC".to_owned()),
            (2, "WA".to_owned()),
        ]),
        (2, 2)
    );

    assert_eq!(
        solve(vec![
            (7777, "AC".to_owned()),
            (7777, "AC".to_owned()),
            (7777, "AC".to_owned()),
        ]),
        (1, 0)
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
    let (_, m) = parse_line!(usize, usize);
    let ps = (0..m)
        .map(|_| parse_line!(usize, String))
        .collect::<Vec<_>>();

    let (ac, wa) = solve(ps);
    println!("{} {}", ac, wa);
}
