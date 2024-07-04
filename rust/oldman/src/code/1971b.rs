// https://codeforces.com/contest/1971/problem/B

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
        let mut s: Vec<char> = scan.next::<String>().chars().collect();

        let mut ans = false;
        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                (s[i], s[i - 1]) = (s[i - 1], s[i]);
                ans = true;
                break;
            }
        }

        if !ans {
            writeln!(out, "{}", "NO").ok();
        } else {
            writeln!(out, "{}", "YES").ok();
            writeln!(out, "{}", s.iter().collect::<String>()).ok();
        }
    }
}
