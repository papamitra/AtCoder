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
        line.trim_right().to_string()
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
    let mut k = parse_line!(usize);

    if k <= 9 {
        println!("{}", k);
        return;
    }

    let mut oldv = (1..10).map(|c| c.to_string()).collect::<Vec<_>>();
    let mut newv = Vec::new();
    let mut cnt = 9;

    loop {
        for n in oldv.iter() {
            let n = n.chars().collect::<Vec<_>>();
            let l = n[n.len() - 1].to_digit(10).unwrap();

            for r in 0..10 {
                if std::cmp::max(l, r) - std::cmp::min(l, r) <= 1 {
                    let mut n = n.clone();
                    n.push(std::char::from_digit(r, 10).unwrap());
                    let n = n.into_iter().collect::<String>();
                    cnt += 1;
                    if cnt == k {
                        println!("{}", n);
                        return;
                    }

                    newv.push(n);
                }
            }
        }

        oldv = newv;
        newv = Vec::new();
    }
}
