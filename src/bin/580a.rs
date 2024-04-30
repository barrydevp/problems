// https://codeforces.com/problemset/problem/580/a

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

    let a = (0..n).map(|_| scan.next::<u32>()).collect::<Vec<u32>>();

    let mut max = 1;
    let mut cur = 1;
    for i in 1..n {
        cur = if a[i - 1] > a[i] {
            max = max.max(cur);
            1
        } else {
            cur + 1
        };
    }
    max = max.max(cur);

    writeln!(out, "{}", max).ok();
}
