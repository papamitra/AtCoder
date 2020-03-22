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
        line.trim_right();
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

fn is_kaibun(s: &String) -> bool {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();

    for i in 0..n {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }

    true
}

fn main() {
    let s = read_line!();
    let n = s.len();
    println!(
        "{}",
        if is_kaibun(&s)
            && is_kaibun(&s[0..((n - 1) / 2)].to_string())
            && is_kaibun(&s[((n + 3) / 2 - 1)..].to_string())
        {
            "Yes"
        } else {
            "No"
        }
    );
}
