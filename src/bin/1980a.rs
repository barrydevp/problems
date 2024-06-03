// https://codeforces.com/contest/1980/problem/A

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

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let (_, m) = (scan.next::<usize>(), scan.next::<usize>());

        let s = scan.next::<String>();

        let mut a = [0; 7];

        for p in s.chars() {
            a[p as usize - 65] += 1;
        }

        let mut ans = 0;
        for v in a {
            if v < m {
                ans += m - v;
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
