fn solve(nstr: &str, k: usize) -> usize {
    if k == 0 {
        return 1;
    }

    let n = nstr.len();
    if n < k {
        return 0;
    }

    let top: usize = nstr[0..1].parse().unwrap();

    if top == 0 {
        solve(&nstr[1..], k)
    } else {
        let mut s = "".to_string();
        for _ in 0..(n - 1) {
            s.push_str("9");
        }

        solve(&s, k) + solve(&nstr[1..], k - 1) + (top - 1) * solve(&s, k - 1)
    }
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
    let nstr = read_line!();
    let k = parse_line!(usize);

    println!("{}", solve(&nstr, k));
}
