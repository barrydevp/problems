// https://oj.vnoi.info/problem/voi21_noel

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, d) = scan!(usize, i32);
    let n2 = n * 2;

    let a = (0..n2).map(|_| scan!(i32)).collect::<Vec<i32>>();

    let mut dp = vec![vec![0; n2 + 1]; n2 + 1];

    let mut max = 0;

    for k in 1..n2 {
        for i in 1..=k {
            for j in (k + 1)..=n2 {
                if (a[i - 1] - a[j - 1]).abs() <= d {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                }
            }
        }
        max = max.max(dp[k][n2]);
    }

    println!("{}", max);
}
