// https://codeforces.com/contest/630/problem/I

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

fn binpow(mut a: u64, mut b: u64) -> u64 {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res *= a;
        }
        a *= a;
        b >>= 1;
    }
    res
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    // let nt = scan.next::<usize>();
    let nt = 1;
    for _ in 0..nt {
        let n = scan.next::<u64>();

        writeln!(
            out,
            "{}",
            2 * 4 * 3 * binpow(4, n - 3)
                + if n > 3 {
                    (n - 3) * 4 * 3 * 3 * binpow(4, n - 4)
                } else {
                    0
                }
        )
        .ok();
    }
}
