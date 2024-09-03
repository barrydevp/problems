// https://codeforces.com/contest/2009/problem/D

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

        let mut p = vec![(0, 0); n];

        let mut a = vec![vec![false; 2 * 100_000 + 1]; 2];
        let mut xx = [0; 2];
        for i in 0..n {
            let x = scan.next::<usize>();
            let y = scan.next::<usize>();
            a[y][x] = true;
            p[i] = (x, y);
            xx[y] += 1;
        }

        let mut ans = 0i64;
        for j in 0..n {
            let v = p[j];
            if a[(v.1 + 1) % 2][v.0] {
                ans += xx[(v.1 + 1) % 2] - 1;
            }
            if v.0 > 0 && v.0 < n && a[(v.1 + 1) % 2][v.0 - 1] && a[(v.1 + 1) % 2][v.0 + 1] {
                ans += 1;
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
