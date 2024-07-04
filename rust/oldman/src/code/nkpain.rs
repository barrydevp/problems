// https://oj.vnoi.info/problem/nkpain

#[macro_use]
extern crate dmoj;

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let a = input.trim().chars().collect::<Vec<char>>();
    let n = a.len();

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 0..n {
        for j in 0..n {
            dp[i + 1][j + 1] = if a[i] == a[n - j - 1] {
                dp[i][j] + 1
            } else {
                dp[i][j + 1].max(dp[i + 1][j])
            };
        }
    }

    let mut r = vec!['a'; dp[n][n]];

    let mut idx = dp[n][n];
    let (mut i, mut j) = (n, n);
    while idx > 0 {
        if a[i-1] == a[n-j] {
            r[idx-1] = a[i-1];
            idx -= 1;
            i -= 1;
            j -= 1;
        } else if dp[i][j] > dp[i - 1][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    for c in r {
        print!("{}", c);
    }
    println!();
}
