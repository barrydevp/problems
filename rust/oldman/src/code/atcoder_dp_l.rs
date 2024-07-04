// https://oj.vnoi.info/problem/atcoder_dp_l

#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);

    let a = (0..n).map(|_| scan!(i64)).collect::<Vec<i64>>();

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        dp[i][i] = a[i - 1];
        for j in (1..i).rev() {
            dp[j][i] = (a[i - 1] - dp[j][i - 1]).max(a[j - 1] - dp[j + 1][i]);
        }
    }

    println!("{}", dp[1][n]);
}
