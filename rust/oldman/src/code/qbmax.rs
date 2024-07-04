// https://oj.vnoi.info/problem/qbmax

use std::cmp::max;

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let mn = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (m, n) = (mn[0], mn[1]);

    let mut a = Vec::with_capacity(n);

    for _ in 0..m {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let s = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        a.push(s)
    }

    let mut dp = vec![vec![0; m]; n];
    for i in 0..m {
        dp[0][i] = a[i][0];
    }

    for i in 1..n {
        for j in 0..m {
            let t = if j > 0 { j - 1 } else { 0 };
            let b = if j < (m - 1) { j + 1 } else { m - 1 };

            dp[i][j] = max(dp[i - 1][t], max(dp[i - 1][j], dp[i - 1][b])) + a[j][i];
        }
    }

    println!("{}", dp[n-1].iter().max().unwrap());
}
