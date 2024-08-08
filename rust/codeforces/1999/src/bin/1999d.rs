// https://codeforces.com/contest/1999/problem/E

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

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let mut s = scan.next_chars();
        let t = scan.next_chars();

        let n = s.len();

        // let mut dp = vec![0; n + 1];

        let mut j = 0;
        for i in 0..n {
            if j < t.len() {
                if (s[i] == t[j] || s[i] == '?') {
                    s[i] = t[j];
                    j += 1;
                }
            } else if s[i] == '?' {
                s[i] = 'a';
            }
        }
        if j < t.len() {
            writeln!(out, "NO").ok();
        } else {
            writeln!(out, "YES").ok();
            writeln!(out, "{}", s.iter().collect::<String>()).ok();
        }
    }
}
