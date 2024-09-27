// https://codeforces.com/contest/2014/problem/B

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

fn pow(k: i64) -> i64 {
    let mut k = k;
    let mut r = 1;
    // for i in 0..k {
    //     r = (r * k) % 2;
    // }
    let mut base = k;
    while k > 0 {
        if k % 2 == 0 {
            base = base * base;
        } else {
            k -= 1;
            r = (r * base) % 2
        }
        k /= 2;
    }

    r
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<i64>();
        let k = scan.next::<i64>();

        // let mut sum = pow(n-k+1);
        let mut sum = 0;
        for i in 0..k {
            sum += (n - i);
        }

        writeln!(out, "{}", if sum % 2 == 0 { "YES" } else { "NO" }).ok();
    }
}
