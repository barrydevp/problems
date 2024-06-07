// https://codeforces.com/contest/1979/problem/A

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
        let a: Vec<usize> = (0..n).map(|_| scan.next::<usize>()).collect();

        let mut k = 1_000_000_000;

        for i in 1..n {
            k = k.min(a[i].max(a[i - 1]) - 1);
        }

        writeln!(out, "{}", k).ok();
    }
}
