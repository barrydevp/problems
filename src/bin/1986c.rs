// https://codeforces.com/contest/1986/problem/C

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
        let (n, m) = (scan.next::<usize>(), scan.next::<usize>());
        let mut s: Vec<_> = scan.next_line().trim().chars().collect();
        let mut ind = (0..m).map(|_| scan.next::<usize>()).collect::<Vec<_>>();
        let mut c: Vec<_> = scan.next_line().trim().chars().collect();
        c.sort();
        ind.sort();

        s[ind[0] - 1] = c[0];
        let mut j = 1;
        for i in 1..m {
            if ind[i] != ind[i - 1] {
                s[ind[i] - 1] = c[j];
                j += 1;
            }
        }

        writeln!(out, "{}", s.iter().collect::<String>()).ok();
    }
}
