// https://codeforces.com/contest/1971/problem/D

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
        let s: Vec<char> = scan.next::<String>().chars().collect();

        let mut ans = 1;

        let mut i = 0;
        let mut centered = 0;
        while i < s.len() {
            if s[i] == '1' {
                while i < s.len() && s[i] == '1' {
                    i += 1;
                }
                if i < s.len() {
                    ans += 1;
                }
            } else {
                while i < s.len() && s[i] == '0' {
                    i += 1;
                }
                if i < s.len() {
                    centered = 1;
                    ans += 1;
                }
            }
        }
        writeln!(out, "{}", ans - centered).ok();
    }
}
