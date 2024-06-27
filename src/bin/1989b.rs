// https://codeforces.com/contest/1989/problem/B

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
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }
    fn next_line(&mut self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let a = scan.next_chars();
        let b = scan.next_chars();

        // let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

        // for i in 1..=a.len() {
        //     for j in 1..=b.len() {
        //         dp[i][j] = if a[i - 1] == b[j - 1] {
        //             dp[i - 1][j - 1] + 1
        //         } else {
        //             dp[i - 1][j]
        //         };
        //     }
        // }
        //
        // writeln!(out, "{}", a.len() - dp[a.len()][b.len()] + b.len()).ok();

        let mut ans = 0;
        for k in 0..b.len() {
            let mut c = 0;
            let mut i = 0;
            let mut j = k;
            while i < a.len() && j < b.len() {
                if a[i] == b[j] {
                    c += 1;
                    j += 1;
                }
                i += 1;
            }
            ans = max(ans, c);
        }

        writeln!(out, "{}", a.len() - ans + b.len()).ok();
    }
}
