// https://oj.vnoi.info/problem/qbsquare
// editorial: https://vietcodes.github.io/code/161/index.html

#[macro_use]
extern crate dmoj;

fn main() {
    let (m, n) = scan!(usize, usize);

    let mut a = vec![vec![2; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            a[i][j] = scan!(usize);
        }
    }

    // let mut x = vec![vec![0; n + 1]; m + 1];
    // let mut y = vec![vec![0; n + 1]; m + 1];
    // let mut dp = vec![vec![0; n + 1]; m + 1];
    //
    // let mut max = 1;
    //
    // for i in 1..=m {
    //     for j in 1..=n {
    //         x[i][j] = 1 + if a[i][j] == a[i][j - 1] {
    //             x[i][j - 1]
    //         } else {
    //             0
    //         };
    //         y[i][j] = 1 + if a[i][j] == a[i - 1][j] {
    //             y[i - 1][j]
    //         } else {
    //             0
    //         };
    //
    //         dp[i][j] = if a[i][j] == a[i - 1][j - 1] {
    //             (1 + dp[i - 1][j - 1]).min(x[i][j].min(y[i][j]))
    //         } else {
    //             1
    //         };
    //
    //         max = max.max(dp[i][j]);
    //     }
    // }

    let mut dp = vec![vec![0; n + 1]; m + 1];

    let mut max = 1;

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = 1 + if a[i][j] == a[i - 1][j - 1]
                && a[i][j] == a[i][j - 1]
                && a[i][j] == a[i - 1][j]
            {
                dp[i - 1][j - 1].min(dp[i][j - 1].min(dp[i - 1][j]))
            } else {
                0
            };

            max = max.max(dp[i][j]);
        }
    }

    println!("{}", max);
}
