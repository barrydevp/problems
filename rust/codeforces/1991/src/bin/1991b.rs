// https://codeforces.com/contest/1991/problem/B

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
        let n = scan.next::<usize>();
        let b = scan.next_vec::<usize>(n - 1);
        let mut a = vec![0; n];
        a[0] = b[0];

        let mut ans = true;
        for i in 1..(n - 1) {
            a[i] = b[i - 1] | b[i];
            if a[i] & a[i - 1] != b[i - 1] {
                ans = false;
            }
        }
        a[n - 1] = b[n - 2];

        if !ans {
            writeln!(out, "{}", -1).ok();
        } else {
            for i in 0..n {
                write!(out, "{} ", a[i]).ok();
            }
            writeln!(out).ok();
        }
    }
}
