// https://codeforces.com/contest/2020/problem/C

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
    let max = i64::pow(10, 18);

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let b = scan.next::<i64>();
        let c = scan.next::<i64>();
        let d = scan.next::<i64>();

        let mut ans = 0i64;

        let mut l = 0;
        let mut r = 10;
        while l < r {
            let m = l + (r - l) / 2;
            let v = (m | b) - (m & c);
            println!("l: {}, r: {}, m: {}, v: {}", l, r, m, v);
            // break;
            if v >= d {
                r = m;
            } else {
                l = m + 1;
            }
        }
        println!("l: {}, r: {}, v: {}", l, r, (l|b) - (l&c));

        if (l | b) - (l & c) == d {
            ans = l;
        } else {
            ans = -1;
        }

        writeln!(out, "{}", ans).ok();
    }
}
