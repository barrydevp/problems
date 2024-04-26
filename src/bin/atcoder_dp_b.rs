// https://oj.vnoi.info/problem/atcoder_dp_b

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let nk = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (n, k) = (nk[0], nk[1]);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let h = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut dp = vec![0; n + 1];

    for i in 2..=n {
        dp[i] = dp[i - 1] + (h[i - 1] - h[i - 2]).abs();
        let l = if i <= k { 1 } else { i - k };
        let r = i - 1;
        for j in l..=r {
            dp[i] = dp[i].min(dp[j] + (h[i - 1] - h[j - 1]).abs());
        }
    }

    println!("{}", dp[n]);
}
