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
    let mut a = vec![0; 2 * 100_000 + 1];
    let mut d = 1;
    let mut c = 1;
    for i in 1..=2 * 100_000 {
        if d * 3 == i {
            d = i;
            c += 1;
        }
        a[i] = a[i - 1] + c;
    }

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let l = scan.next::<usize>();
        let r = scan.next::<usize>();

        let mut ans = 0u64;
        let mut ll = l;
        while ll > 0 {
            ans += 2;
            ll /= 3;
        }

        writeln!(out, "{}", ans + a[r] - a[l]).ok();
    }
}
