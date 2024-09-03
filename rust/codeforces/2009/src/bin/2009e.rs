// https://codeforces.com/contest/2009/problem/E

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
        let n = scan.next::<i64>();
        let k = scan.next::<i64>();

        let mut l = k;
        let mut r = k + n - 1;
        while l < r {
            let m = (l + r) / 2;
            let a = (m + k) * (m - k + 1) / 2;
            let b = (k + n + m) * (k + n - (m + 1)) / 2;
            if a >= b {
                r = m;
            } else {
                l = m + 1;
            }
        }
        // println!("{}", l);
        let a = (l + k) * (l - k + 1) / 2;
        let b = (k + n + l) * (k + n - (l + 1)) / 2;
        let mut ans = a.abs_diff(b);

        let l = l + 1;
        // println!("{}", l);
        if l >= k && l < k + n - 1 {
            let a = (l + k) * (l - k + 1) / 2;
            let b = (k + n + l) * (k + n - (l + 1)) / 2;
            ans = min(ans, a.abs_diff(b));
        }

        let l = l - 2;
        // println!("{}", l);
        if l >= k {
            let a = (l + k) * (l - k + 1) / 2;
            let b = (k + n + l) * (k + n - (l + 1)) / 2;
            ans = min(ans, a.abs_diff(b));
        }

        let l = l - 1;
        // println!("{}", l);
        if l >= k {
            let a = (l + k) * (l - k + 1) / 2;
            let b = (k + n + l) * (k + n - (l + 1)) / 2;
            ans = min(ans, a.abs_diff(b));
        }

        writeln!(out, "{}", ans).ok();
    }
}
