fn solve(ss: Vec<Vec<char>>) -> usize {
    let h = ss.len();
    let w = ss[0].len();

    let mut m = 0;
    for y in 0..h {
        for x in 0..w {
            if ss[y][x] != '#' {
                let c = _solve((y, x), &ss);
                if m < c {
                    m = c;
                }
            }
        }
    }

    m
}

fn _solve(start: (usize, usize), ss: &Vec<Vec<char>>) -> usize {
    let h = ss.len();
    let w = ss[0].len();

    let mut costs = vec![vec![std::usize::MAX; w]; h];

    let mut pos = vec![(0, start)];

    loop {
        if pos.is_empty() {
            break;
        }

        if let Some((c, (y, x))) = pos.pop() {
            if c < costs[y][x] {
                costs[y][x] = c;
                if y > 0 {
                    if ss[y - 1][x] != '#' {
                        pos.push((c + 1, (y - 1, x)));
                    }
                }

                if y + 1 < h {
                    if ss[y + 1][x] != '#' {
                        pos.push((c + 1, (y + 1, x)));
                    }
                }

                if x > 0 {
                    if ss[y][x - 1] != '#' {
                        pos.push((c + 1, (y, x - 1)));
                    }
                }

                if x + 1 < w {
                    if ss[y][x + 1] != '#' {
                        pos.push((c + 1, (y, x + 1)));
                    }
                }
            }
        }
    }

    let mut m = 0;
    for i in 0..h {
        for j in 0..w {
            if costs[i][j] < std::usize::MAX && m < costs[i][j] {
                m = costs[i][j];
            }
        }
    }

    m
}

#[test]
fn solve_test() {
    assert_eq!(
        solve(vec![
            "...".chars().collect::<Vec<_>>(),
            "...".chars().collect::<Vec<_>>(),
            "...".chars().collect::<Vec<_>>(),
        ]),
        4
    );

    assert_eq!(
        solve(vec![
            "...#.".chars().collect::<Vec<_>>(),
            ".#.#.".chars().collect::<Vec<_>>(),
            ".#...".chars().collect::<Vec<_>>(),
        ]),
        10
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

    let ss = (0..h)
        .map(|_| read_line!().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    println!("{}", solve(ss));
}
