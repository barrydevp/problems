// https://codeforces.com/contest/1980/problem/B

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
        let (n, f) = (scan.next::<usize>(), scan.next::<usize>());
        let k = scan.next::<usize>();

        let mut a = [0; 101];

        let mut r = 0;
        for i in 1..=n {
            let v = scan.next::<usize>();
            if i == f {
                r = v;
            }

            a[v] += 1;
        }

        let mut d = 0;
        let mut l = 0;
        for i in (0..100).rev() {
            d += a[i];
            if d >= k {
                l = i;
                break;
            }
        }
        // writeln!(out, "{:?}", a);
        // writeln!(out, "{} {} {}", d, l, r).ok();

        if l > r {
            writeln!(out, "{}", "NO").ok();
        } else if l < r || d == k {
            writeln!(out, "{}", "YES").ok();
        } else {
            writeln!(out, "{}", "MAYBE").ok();
        }
    }
}
