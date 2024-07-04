// https://oj.vnoi.info/problem/atcoder_dp_e

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, w) = scan!(usize, usize);

    let mut a = vec![(0, 0); n + 1];

    let mut max_v = 0;
    for i in 1..=n {
        a[i] = scan!(usize, usize);
        max_v += a[i].1;
    }

    let mut dp = vec![w + 1; max_v + 1];
    dp[0] = 0;

    max_v = 0;
    for i in 1..=n {
        max_v += a[i].1;
        for j in (a[i].1..=max_v).rev() {
            dp[j] = dp[j].min(dp[j - a[i].1] + a[i].0)
        }
    }

    for i in (0..=max_v).rev() {
        if dp[i] <= w {
            println!("{}", i);
            break;
        }
    }
}
