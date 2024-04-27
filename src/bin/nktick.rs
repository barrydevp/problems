// https://oj.vnoi.info/problem/nktick

use std::cmp::min;

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let n = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let r = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut dp = vec![0; n + 1];
    dp[1] = t[0];

    for i in 2..=n {
        dp[i] = min(dp[i - 1] + t[i - 1], dp[i - 2] + r[i - 2]);
    }

    println!("{}", dp[n]);
}
