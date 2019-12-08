fn solve(xys: Vec<Vec<(usize, usize)>>) -> usize {
    let mut anss = Vec::new();
    anss.push(0);

    let n = xys.len();
    'outer: for i in (0..2usize.pow(n as u32)).rev() {
        let mut aa: Vec<Option<usize>> = vec![None; n];
        let mut ans = 0;
        for j in 0..n {
            if (i & (1 << j)) > 0 {
                if let Some(y) = aa[j] {
                    if y == 0 {
                        continue 'outer;
                    }
                }

                ans += 1;
                for &(x, y) in xys[j].iter() {
                    if let Some(yp) = aa[x - 1] {
                        if yp != y {
                            continue 'outer;
                        }
                    } else {
                        aa[x - 1] = Some(y);
                    }
                }
                aa[j] = Some(1);
            } else {
                if let Some(yp) = aa[j] {
                    if yp != 0 {
                        continue 'outer;
                    }
                }
                aa[j] = Some(0);
            }
        }
        anss.push(ans);
    }

    *anss.iter().max().unwrap()
}

#[test]
fn solve_test() {
    assert_eq!(solve(vec![vec![(2, 1)], vec![(1, 1)], vec![(2, 0)]]), 2);
    assert_eq!(solve(vec![vec![(2, 0)], vec![(1, 0)]]), 1);
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

    let mut xys = Vec::new();

    for _ in 0..n {
        let a = parse_line!(usize);

        let v = (0..a)
            .map(|_| parse_line!(usize, usize))
            .collect::<Vec<_>>();
        xys.push(v);
    }

    println!("{}", solve(xys));
}
