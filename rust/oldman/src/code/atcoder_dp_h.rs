// https://oj.vnoi.info/problem/atcoder_dp_h

#[macro_use]
extern crate dmoj;

const MOD: usize = 1000000007;

fn main() {
    let (h, w) = scan!(usize, usize);

    let mut dp = vec![vec![0; w + 1]; h + 1];
    // simple trick to avoid assigning 1 or checking for the first (1,1) box
    dp[1][0] = 1;
    for i in 1..=h {
        // escape the newline
        scan!(char);

        for j in 1..=w {
            let c = scan!(char);
            dp[i][j] = if c == '#' {
                0
            } else {
                (dp[i - 1][j] + dp[i][j - 1]) % MOD
            };
        }
    }

    println!("{}", dp[h][w]);
}
