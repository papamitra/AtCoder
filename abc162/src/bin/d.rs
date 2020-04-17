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
    let n = parse_line!(usize);
    let s = read_line!().chars().collect::<Vec<_>>();

    let mut rn = 0u64;
    let mut gn = 0u64;
    let mut bn = 0u64;

    for &c in s.iter() {
        if c == 'R' {
            rn += 1;
        }

        if c == 'G' {
            gn += 1;
        }

        if c == 'B' {
            bn += 1;
        }
    }

    let mut cnt = 0;
    for i in 1..(n - 1) {
        let mut j = 1;
        loop {
            if s[i - j] != s[i] && s[i] != s[i + j] && s[i - j] != s[i + j] {
                cnt += 1;
            }

            if i - j == 0 || i + j == n - 1 {
                break;
            }

            j += 1;
        }
    }

    println!("{}", rn * gn * bn - cnt);
}

/*
fn main() {
    let n = parse_line!(usize);
    let s = read_line!().chars().collect::<Vec<_>>();

    let mut cnt = 0;
    for i in 0..(n - 2) {
        let si = s[i];
        for j in i..(n - 1) {
            let sj = s[j];
            if si == sj {
                continue;
            }
            for k in j..n {
                let sk = s[k];
                if si != sj && sj != sk && si != sk && (j - i != k - j) {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
*/
