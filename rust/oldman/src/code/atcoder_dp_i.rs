// https://oj.vnoi.info/problem/atcoder_dp_i

#[macro_use]
extern crate dmoj;

fn main() {
    // let n = scan!(usize);
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let n = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let a = input
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; n + 1]; n + 1];
    dp[1][0] = 1.0 - a[0];
    dp[1][1] = a[0];

    for i in 2..=n {
        dp[i][0] = dp[i - 1][0] * (1.0 - a[i - 1]);
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] * a[i - 1] + dp[i - 1][j] * (1.0 - a[i - 1]);
        }
    }

    let mut p: f64 = 0.0;

    for i in n / 2 + 1..=n {
        p += dp[n][i];
    }

    println!("{}", p);
}
