// https://codeforces.com/contest/1999/problem/B

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
    #[allow(dead_code)]
    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }
    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        input
    }
}

fn cmp(a: usize, b: usize) -> i32 {
    match Some(a) {
        Some(x) if x > b => 1,
        Some(x) if x < b => -1,
        _ => 0,
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let a1 = scan.next::<usize>();
        let a2 = scan.next::<usize>();
        let b1 = scan.next::<usize>();
        let b2 = scan.next::<usize>();

        let mut ans = 0;

        if (cmp(a1, b1) + cmp(a2, b2)) > 0 {
            ans += 2;
        }
        if (cmp(a2, b1) + cmp(a1, b2)) > 0 {
            ans += 2;
        }

        writeln!(out, "{}", ans).ok();
    }
}
