// https://codeforces.com/contest/1971/problem/C

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
        let n: Vec<i32> = (0..4).map(|_| scan.next::<i32>()).collect();
        let (a, b, c, d) = (n[0], n[1], n[2], n[3]);
        // let (a, b) = if n[0] < n[1] {
        //     (n[0], n[1])
        // } else {
        //     (n[1], n[0])
        // };
        // let (c, d) = if n[2] < n[3] {
        //     (n[2], n[3])
        // } else {
        //     (n[3], n[2])
        // };

        let ans = (a - c) * (a - d) * (b - c) * (b - d) < 0;

        writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
    }
}
