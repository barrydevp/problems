// https://oj.vnoi.info/problem/atcoder_dp_a

// dp[n] = max(dp[n-1] + |h[n]-h[n-1]|, |dp[n-2]| + |h[n]-h[n-2]|)

use std::cmp::min;

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let n = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let h = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut dp = vec![0; n + 1];
    dp[2] = (h[1] - h[0]).abs();

    for i in 3..=n {
        dp[i] = min(
            dp[i - 1] + (h[i-1] - h[i-2]).abs(),
            dp[i - 2] + (h[i-1] - h[i-3]).abs(),
        );
    }

    println!("{}", dp[n]);
}
