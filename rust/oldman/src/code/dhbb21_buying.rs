// https://oj.vnoi.info/problem/dhbb21_buying
// Editorial: https://youtu.be/zIpewcucOe0?t=4395

#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);

    let mut a = (0..n)
        .map(|_| scan!(i64, i64, i64))
        .collect::<Vec<(i64, i64, i64)>>();

    a.sort_by(|x, y| (x.1 - x.0).cmp(&(y.1 - y.0)));

    // println!("{:?}", a);
    let mut dp = vec![vec![vec![0xfffffffffff; 2]; n + 1]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for nc in 0..=i {
            // for na in 0..2 {
            //     if dp[i][nc][na] < 0xfffffffffff {
            //         dp[i + 1][nc][1] = dp[i + 1][nc][1].min(dp[i][nc][na] + a[i].0);
            //
            //         if na == 0 {
            //             dp[i + 1][nc][na] =
            //                 dp[i + 1][nc][na].min(dp[i][nc][na] + a[i].1 - (i - nc) as i64);
            //         }
            //
            //         dp[i + 1][nc + 1][na] =
            //             dp[i + 1][nc + 1][na].min(dp[i][nc][na] + a[i].2 - nc as i64);
            //     }
            // }

            // from store A
            dp[i + 1][nc][1] = dp[i + 1][nc][1].min(dp[i][nc][0].min(dp[i][nc][1]) + a[i].0);

            // from store B
            dp[i + 1][nc][0] = dp[i + 1][nc][0].min(dp[i][nc][0] + a[i].1 - (i - nc) as i64);

            // from store C
            dp[i + 1][nc + 1][0] = dp[i][nc][0] + a[i].2 - nc as i64;
            dp[i + 1][nc + 1][1] = dp[i][nc][1] + a[i].2 - nc as i64;
        }
    }

    let mut min = 0xfffffffffff;
    for i in 0..=n {
        min = min.min(dp[n][i][0]).min(dp[n][i][1]);
    }

    println!("{}", min);
}
