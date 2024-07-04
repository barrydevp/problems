// https://oj.vnoi.info/problem/atcoder_dp_m

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, k) = scan!(usize, usize);

    let a = (0..n).map(|_| scan!(usize)).collect::<Vec<usize>>();

    // let mut dp = vec![vec![-1; k + 1]; n + 1];
    // for i in 0..=a[0] {
    //     dp[1][i] = 1;
    // }
    //
    // for i in 2..=n {
    //     for j in 0..=a[i - 1] {
    //         for h in j..=k {
    //             dp[i][h] = (dp[i][h] + dp[i - 1][h - j]) % 1_000_000_007;
    //         }
    //     }
    // }

    // println!("{:?}", dp);

    // println!("{}", rec(&a, &mut dp, n, k));

    let mut dp = vec![vec![0_i32; k + 1]; 2];
    dp[0][0] = 1;

    for i in 1..=n {
        let pi = (i - 1) % 2;
        let ci = i % 2;
        for j in 1..=k {
            dp[pi][j] = (dp[pi][j] + dp[pi][j - 1]) % 1_000_000_007;
        }
        for j in 0..=k {
            dp[ci][j] = ((dp[pi][j]
                - if j > a[i - 1] {
                    dp[pi][j - a[i - 1] - 1]
                } else {
                    0
                })
                + 1_000_000_007)
                % 1_000_000_007;
        }
    }

    println!("{}", dp[n % 2][k]);
}
