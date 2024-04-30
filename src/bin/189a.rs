// https://codeforces.com/problemset/problem/189/a

#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();

    let a = scan.next::<usize>();
    let b = scan.next::<usize>();
    let c = scan.next::<usize>();

    let mut dp = vec![-(n as i32); n + 1];
    dp[0] = 0;

    for j in (a.min(b.min(c)))..=n {
        if j >= a {
            dp[j] = dp[j - a] + 1;
        }

        if j >= b {
            dp[j] = dp[j].max(dp[j - b] + 1);
        }

        if j >= c {
            dp[j] = dp[j].max(dp[j - c] + 1);
        }
    }

    writeln!(out, "{}", dp[n]).ok();
}
