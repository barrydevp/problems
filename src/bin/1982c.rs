// https://codeforces.com/contest/1982/problem/C

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
    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
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
        let (n, l, r) = (scan.next::<usize>(), scan.next::<i32>(), scan.next::<i32>());

        let a = scan.next_vec::<i32>(n);

        let mut ans = 0;
        let mut il = 0;
        let mut acc = 0;
        for i in 0..n {
            acc += a[i];

            while acc > r {
                acc -= a[il];
                il += 1;
            }

            if acc >= l {
                acc = 0;
                il = i + 1;
                ans += 1;
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
