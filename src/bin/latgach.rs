// https://oj.vnoi.info/problem/latgach

#[macro_use]
extern crate dmoj;

fn main() {
    let t = scan!(usize);
    let mut dp = vec![0_u128; 100 + 1];
    let mut last_n = 1;

    for _ in 1..=t {
        let n = scan!(usize);
        dp[0] = 1;
        dp[1] = 1;

        if n > last_n {
            for i in (last_n + 1)..=n {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            last_n = n;
        }

        println!("{}", dp[n]);
    }
}
