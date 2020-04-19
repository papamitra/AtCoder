use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ps: [f64; n],
    };

    // i枚なげてj枚表になる確率
    let mut dp = vec![vec![0f64; n + 1]; n + 1];

    dp[0][0] = 1.0;

    for i in 1..=n {
        dp[i][0] = dp[i - 1][0] * (1.0 - ps[i - 1]);

        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] * ps[i - 1] + dp[i - 1][j] * (1.0 - ps[i - 1]);
        }
    }

    let mut sum = 0.0;

    for i in ((n + 1) / 2)..=n {
        sum += dp[n][i];
    }

    println!("{}", sum);
}
