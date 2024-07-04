// https://oj.vnoi.info/problem/ctnbulls

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, k) = scan!(usize, usize);

    let mut dp = vec![0; n + 1];
    (1..=k + 1).for_each(|i| {
        dp[i] = i + 1;
    });

    (k + 2..=n).for_each(|i| dp[i] = (dp[i - 1] + dp[i - k - 1]) % 2111992);
    // println!("{:?}", dp);

    println!("{}", dp[n]);
}
