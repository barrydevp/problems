// https://oj.vnoi.info/problem/nkrez

#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);

    let mut a = vec![vec![]; 30001];

    let mut max_e = 0;
    for _ in 1..=n {
        let (s, e) = scan!(usize, usize);
        max_e = max_e.max(e);
        a[e].push(s);
    }

    let mut dp = vec![0; max_e+1];

    for i in 1..=max_e {
        dp[i] = dp[i - 1];
        a[i].iter().for_each(|&j| {
            dp[i] = dp[i].max(dp[j] + i - j);
        });
    }

    println!("{}", dp[max_e]);
}
