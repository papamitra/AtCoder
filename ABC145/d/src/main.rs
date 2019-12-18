const FACT_MAX: usize = 2000000;
const MOD: i64 = 1000000007;

#[allow(dead_code)]
fn pow_mod(base: i64, power: i64, modulo: i64) -> i64 {
    let mut base = base;
    let mut power = power;
    let mut ans = 1;
    while power > 0 {
        if power & 1 == 1 {
            ans = ans * base % modulo;
        }
        power >>= 1;
        base = base * base % modulo;
    }
    ans
}

thread_local! {
    #[allow(unused)]
    pub static FACT: Vec<i64> = {
        let mut fac: Vec<i64> = vec![1; FACT_MAX+1];

        for i in 1..FACT_MAX+1 {
            fac[i] = (fac[i-1] * i as i64) as i64 % MOD;
        }
        fac
    };

    #[allow(unused)]
    pub static RFACT: Vec<i64> = {
        let mut rfac = vec![1; FACT_MAX+1];
        rfac[FACT_MAX] = FACT.with(|fact| {
            pow_mod(fact[FACT_MAX], MOD-2, MOD)
        });

        for i in (1..FACT_MAX).rev() {
            rfac[i] = (rfac[i+1] * (i+1) as i64) % MOD;
        }
        rfac
    }
}

#[allow(dead_code)]
fn combi(n: usize, m: usize) -> i64 {
    FACT.with(|fact| RFACT.with(|rfact| fact[n] * rfact[n - m] % MOD * rfact[m] % MOD))
}

fn solve(x: usize, y: usize) -> i64 {
    if x < y {
        return solve(y, x);
    }

    if x < (y / 2) {
        return 0;
    }
    if y < (x / 2) {
        return 0;
    }

    if (x + y) % 3 != 0 {
        return 0;
    }

    let d = x - y;
    let r = (y - d) / 3;
    let q = r + d;

    combi(r + q, r)
}

#[test]
fn solve_test() {
    assert_eq!(solve(3, 3), 2);
    assert_eq!(solve(999999, 999999), 151840682);
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
    let (x, y) = parse_line!(usize, usize);
    println!("{}", solve(x, y));
}
