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

const FACT_MAX: usize = 300000;
const MOD: i64 = 998244353;

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
    if n < m {
        return 0;
    }
    FACT.with(|fact| RFACT.with(|rfact| fact[n] * rfact[n - m] % MOD * rfact[m] % MOD))
}

fn main() {
    let (n, m) = parse_line!(usize, usize);

    println!("{}", combi(n, 2) + combi(m, 2));
}
