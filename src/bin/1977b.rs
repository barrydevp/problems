// https://codeforces.com/contest/1977/problem/B

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

    let t = scan.next::<usize>();

    for _ in 0..t {
        let mut n: u32 = scan.next::<u32>();

        let mut a: Vec<i32> = Vec::with_capacity(32);
        while n > 0 {
            if (n & 1) == 1 && (n & 2) == 2 {
                a.push(-1);
                while n & 2 == 2 {
                    a.push(0);
                    n >>= 1;
                }
                n += 1;
            } else {
                a.push((n & 1) as i32);
            }
            n >>= 1;
        }

        writeln!(out, "{}", a.len()).ok();
        for x in a {
            write!(out, "{} ", x).ok();
        }
        writeln!(out).ok();
    }
}
