// https://codeforces.com/contest/1985/problem/A

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
        let mut a: Vec<char> = scan.next::<String>().chars().collect();
        let mut b: Vec<char> = scan.next::<String>().chars().collect();
        (a[0], b[0]) = (b[0], a[0]);

        writeln!(out, "{} {}", a.iter().collect::<String>(), b.iter().collect::<String>()).ok();
    }
}
