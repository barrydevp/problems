// https://oj.vnoi.info/problem/atcoder_dp_d

use std::cmp::max;

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let nw = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (n, w) = (nw[0], nw[1]);

    let mut a = vec![[0; 2]; n + 1];

    for i in 1..=n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let s = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        a[i] = [s[0], s[1]];
    }

    let mut dp = vec![0; w + 1];

    for i in 1..=n {
        for j in (a[i][0]..=w).rev() {
            dp[j] = max(dp[j], dp[j - a[i][0]] + a[i][1]);
        }
    }

    println!("{}", dp[w]);
}
