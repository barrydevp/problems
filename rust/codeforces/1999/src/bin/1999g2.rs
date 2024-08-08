// https://codeforces.com/contest/1999/problem/G2

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
        // let n = scan.next::<usize>();
        // writeln!(out, "{}", n).ok();

        let mut l = 1;
        let mut r = 999;

        while (r - l) > 2 {
            let a = (r - l) / 3 + l;
            let b = (r - l) / 3 * 2 + l;
            writeln!(out, "? {} {}", a, b).ok();
            out.flush().ok();
            let s = scan.next::<i32>();

            match s {
                v if v == a * b => {
                    l = b;
                }
                v if v == a * (b + 1) => {
                    l = a;
                    r = b;
                }
                v if v == (a + 1) * (b + 1) => {
                    r = a;
                }
                _ => (),
            }
        }

        if r - l == 2 {
            let mid = (l + r) / 2;
            writeln!(out, "? {} {}", mid, mid).ok();
            out.flush().ok();
            let s = scan.next::<i32>();
            if s != mid * mid {
                r = mid;
            }
        }

        writeln!(out, "! {}", r).ok();
        out.flush().ok();
    }
}
