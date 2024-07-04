// https://oj.vnoi.info/problem/atcoder_dp_c

use std::cmp::max;

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let n = input.trim().parse::<usize>().unwrap();

    let mut a = vec![[0; 3]; n];

    for i in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let s = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        a[i] = [s[0], s[1], s[2]];
    }

    let mut dp = vec![[0; 3]; n];
    dp[0] = a[0];

    for i in 1..n {
        dp[i][0] = max(dp[i - 1][1], dp[i - 1][2]) + a[i][0];
        dp[i][1] = max(dp[i - 1][0], dp[i - 1][2]) + a[i][1];
        dp[i][2] = max(dp[i - 1][0], dp[i - 1][1]) + a[i][2];
        // for j in 0..3 {
        //     dp[i][j] = max(dp[i - 1][(j + 1) % 3], dp[i - 1][(j + 2) % 3]) + a[i][j];
        //     // println!("{:?}", dp[i]);
        // }
    }

    println!("{}", max(dp[n - 1][0], max(dp[n - 1][1], dp[n - 1][2])));
}
