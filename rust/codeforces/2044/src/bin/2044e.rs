// https://codeforces.com/contest/2044/problem/E

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
        let k = scan.next::<usize>();
        let l1 = scan.next::<usize>();
        let r1 = scan.next::<usize>();
        let l2 = scan.next::<usize>();
        let r2 = scan.next::<usize>();

        let mut cur = 1;
        let mut ans = 0;
        while cur <= 1_000_000_000 {
            let lx = max(l1, (l2 - 1) / cur + 1);
            let hx = min(r1, r2 / cur);
            if lx <= hx {
                ans += hx - lx + 1;
            }
            cur *= k;
        }

        writeln!(out, "{}", ans).ok();
    }
}
