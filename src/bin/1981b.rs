// https://codeforces.com/contest/1981/problem/B

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
        let (n, m) = (scan.next::<usize>(), scan.next::<usize>());

        let l = if m > n { 0 } else { n - m };
        let r = n + m;

        let mut ans = n;

        for i in (0..32).rev() {
            if (l & (1 << i) != 0) || (r & (1 << i) != 0) || ((l >> i) != (r >> i)) {
                ans |= 1 << i;
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
