// https://oj.vnoi.info/problem/ptrang

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, l) = scan!(usize, usize);

    let a = (0..n).map(|_| scan!(usize)).collect::<Vec<usize>>();

    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        let mut cur = l - a[i - 1];
        dp[i] = cur.max(dp[i - 1]);
        let mut j = i - 1;
        while j > 0 && cur >= a[j - 1] {
            cur -= a[j - 1];
            dp[i] = dp[i].min(cur.max(dp[j - 1]));
            j -= 1;
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[n]);
}
