// https://oj.vnoi.info/problem/atcoder_dp_n
// https://jeffreyxiao.me/blog/knuths-optimization

#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);

    let mut a = vec![0; n + 1];
    for i in 1..=n {
        a[i] += scan!(i32) as i64 + a[i - 1];
    }

    let mut split_pos = vec![vec![0; n + 1]; n + 1];
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 2..=n {
        dp[i - 1][i] = a[i] - a[i - 2];
        split_pos[i - 1][i] = i - 1;
    }
    // println!("{:?}", split_pos);

    for i in 2..=n {
        for j in 1..=(n - i) {
            for k in split_pos[j][j + i - 1]..=split_pos[j + 1][j + i] {
                let val = dp[j][k] + dp[k + 1][j + i];
                if dp[j][j + i] == 0 || val < dp[j][j + i] {
                    dp[j][j + i] = val;
                    split_pos[j][j + i] = k;
                }
            }
            dp[j][j + i] += a[j + i] - a[j - 1];
        }
    }

    // println!("{:?}", split_pos);
    // println!("{:?}", dp);

    println!("{}", dp[1][n]);
}
