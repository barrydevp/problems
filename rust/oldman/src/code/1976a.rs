// https://codeforces.com/contest/1976/problem/A

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
        let _ = scan.next::<usize>();
        let s = scan.next::<String>().chars().collect::<Vec<char>>();

        let mut is_ok = true;
        for i in 1..s.len() {
            if s[i] < s[i - 1] {
                is_ok = false;
                break;
            }
        }

        writeln!(out, "{}", if is_ok { "Yes" } else { "No" }).ok();
    }
}
