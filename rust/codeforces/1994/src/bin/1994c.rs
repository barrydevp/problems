// https://codeforces.com/contest/1994/problem/C

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
    #[allow(dead_code)]
    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }
    #[allow(dead_code)]
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
        let n = scan.next::<usize>();
        let x = scan.next::<usize>() as u64;

        let mut p = vec![0; n + 1];
        (1..=n).for_each(|i| {
            p[i] = scan.next::<usize>() as u64 + p[i - 1];
        });

        let mut dp = vec![0; n + 3];

        for i in (1..=n).rev() {
            let mut l = i;
            let mut r = n + 1;
            while l < r {
                let m = (l + r) / 2;
                if p[m] > (x + p[i - 1]) {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            // upper_bound in c++ will return .end() if not found
            // and .end() is not the last element, it is last+1 element
            // so that why we need r = n + 1 in the first place
            // and because m = (l+r)/2, so m never reach n+1
            dp[i] = l - i + dp[l + 1];
        }

        let mut ans = 0;
        (1..=n).for_each(|i| {
            ans += dp[i];
        });

        writeln!(out, "{}", ans).ok();
    }
}
