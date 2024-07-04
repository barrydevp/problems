// https://codeforces.com/contest/1976/problem/B

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
        let n = scan.next::<usize>();
        let a = (0..n).map(|_| scan.next::<u64>()).collect::<Vec<u64>>();
        let b = (0..n + 1).map(|_| scan.next::<u64>()).collect::<Vec<u64>>();

        let mut r = 1;
        let mut min = u64::MAX;
        for i in 0..n {
            r += a[i].abs_diff(b[i]);
            if (a[i] <= b[n] && b[n] <= b[i]) || (b[i] <= b[n] && b[n] < a[i]) {
                min = 0;
            }
            min = min.min(a[i].abs_diff(b[n])).min(b[i].abs_diff(b[n]));
        }
        r += min;

        writeln!(out, "{}", r).ok();
    }
}
