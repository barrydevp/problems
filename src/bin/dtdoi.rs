// https://oj.vnoi.info/problem/dtdoi

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, mut s) = scan!(usize, usize);

    let mut a = vec![0; n];

    let mut max = 0;
    (0..n).for_each(|i| {
        a[i] = scan!(usize);
        max = max.max(a[i]);
    });

    let mut r = 0;

    if s > 10000 {
        r = (s - 10000) / max;
        s -= max * r;
    }

    let mut dp = vec![1_000_000_000; 10000 + max];
    dp[0] = 0;

    for i in a {
        (i..=s).for_each(|j| {
            dp[j] = dp[j].min(dp[j - i] + 1);
        });
    }

    println!("{}", dp[s] + r);
}
