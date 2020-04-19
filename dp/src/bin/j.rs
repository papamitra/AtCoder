// https://qiita.com/drken/items/03c7db44ccd27820ea0d#j-%E5%95%8F%E9%A1%8C---sushi
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };

    let mut dp = vec![vec![vec![-1f64; 310]; 310]; 310];
    dp[0][0][0] = 0f64;

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    for a in aa {
        if a == 1 {
            i += 1;
        }

        if a == 2 {
            j += 1;
        }

        if a == 3 {
            k += 1;
        }
    }

    println!("{}", solve(i, j, k, &mut dp, n));
}

fn solve(i: usize, j: usize, k: usize, dp: &mut Vec<Vec<Vec<f64>>>, n: usize) -> f64 {
    if dp[i][j][k] >= 0f64 {
        return dp[i][j][k];
    }

    let mut res = 0f64;

    if i > 0 {
        res += solve(i - 1, j, k, dp, n) * i as f64;
    }
    if j > 0 {
        res += solve(i + 1, j - 1, k, dp, n) * j as f64;
    }
    if k > 0 {
        res += solve(i, j + 1, k - 1, dp, n) * k as f64;
    }

    res += n as f64;
    res *= 1.0 / (i + j + k) as f64;

    dp[i][j][k] = res;
    res
}
