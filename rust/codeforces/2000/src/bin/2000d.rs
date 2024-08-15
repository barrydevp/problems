// https://codeforces.com/contest/2000/problem/D

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
        let mut a = vec![0; n + 1];
        for i in 1..=n {
            a[i] = scan.next::<i64>() + a[i - 1];
        }
        let s = scan.next_chars();

        let mut ans = 0;
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            if s[l] == 'L' {
                while r > l && s[r] == 'L' {
                    r -= 1;
                }

                if s[r] == 'R' {
                    ans += a[r + 1] - a[l];
                    r -= 1;
                }
            }
            l += 1;
        }

        writeln!(out, "{}", ans).ok();
    }
}
