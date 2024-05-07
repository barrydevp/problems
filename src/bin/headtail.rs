// https://oj.vnoi.info/problem/headtail

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, s) = scan!(usize, usize);

    let mut dp = vec![vec![0_u128; (n << 1) + 1]; s + 1];
    dp[1][0] = 1;
    dp[1][1] = 1;

    let mut r = 0;
    for i in 2..=s {
        dp[i][0] = dp[i - 1][0];
        dp[i][1] = dp[i - 1][0] + dp[i - 1][1];
        for j in 2..=(n << 1).min(i) {
            if j % 2 == 0 {
                dp[i][0] += dp[i - 1][j];
            } else {
                dp[i][1] += dp[i - 1][j];
            };
            dp[i][j] = dp[i - 1][j - 1];
            if j == (n << 1) {
                r += dp[i][j];
                dp[i][0] -= dp[i - 1][j];
            }
        }
        // dp[i][0] += dp[i - 2][0] * 3 - dp[i - 1][1];
    }
    // println!("{:?}", dp);

    println!("1");
    println!("1 {}", r);
}
