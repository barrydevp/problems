// https://codeforces.com/contest/1979/problem/C

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

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();

        let k: Vec<usize> = (0..n).map(|_| scan.next::<usize>()).collect();

        let mut lcma = k[0];
        (1..n).for_each(|i| {
            lcma = lcm(lcma, k[i]);
        });

        let mut ans = vec![0; n];

        let mut total = 0;
        for i in 0..n {
            ans[i] = lcma / k[i];
            total += ans[i];
        }

        if total >= lcma {
            writeln!(out, "{}", -1).ok();
        } else {
            (0..n).for_each(|i| {
                write!(out, "{} ", ans[i]).ok();
            });
            writeln!(out).ok();
        }
    }
}
