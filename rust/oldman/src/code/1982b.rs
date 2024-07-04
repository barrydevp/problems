// https://codeforces.com/contest/1982/problem/B

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
        let (mut x, y, mut k) = (scan.next::<i32>(), scan.next::<i32>(), scan.next::<i32>());

        while k > 0 {
            let mut z = (x / y) + 1;
            let k1 = k - (z * y - x);
            if k1 < 0 {
                x = x + k;
            } else {
                x = z;
                while x % y == 0 {
                    x = x / y;
                }
            }
            k = k1;
            if x == 1 {
                x = 1 + (k % (y - 1));
                break;
            }
            // println!("{}, {}, {}", x, y, k);
        }

        writeln!(out, "{}", x).ok();
    }
}
