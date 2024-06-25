// https://codeforces.com/contest/1982/problem/A

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
        let (x1, y1) = (scan.next::<usize>(), scan.next::<usize>());
        let (x2, y2) = (scan.next::<usize>(), scan.next::<usize>());

        let mut ans = true;
        if x1 > y1 {
            if y2 > x2 {
                ans = false;
            }
        } else {
            if x2 > y2 {
                ans = false;
            }
        }

        writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
    }
}
