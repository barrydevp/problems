// https://codeforces.com/contest/1976/problem/C

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

    let nt = scan.next::<usize>();

    for _ in 0..nt {
        let n = scan.next::<usize>();
        let m = scan.next::<usize>();
        let s = n + m + 1;
        let a = (0..s).map(|_| scan.next::<i64>()).collect::<Vec<i64>>();
        let b = (0..s).map(|_| scan.next::<i64>()).collect::<Vec<i64>>();

        let mut p = 0;
        let mut ip = 0;
        let mut np = n + 1;
        while np > 0 {
            if a[ip] > b[ip] || np == (s - ip) {
                p += a[ip];
                np -= 1;
            }
            ip += 1;
        }

        let mut t = 0;
        let mut it = 0;
        let mut nt = m + 1;
        while nt > 0 {
            if a[it] < b[it] || nt == (s - it) {
                t += b[it];
                nt -= 1;
            }
            it += 1;
        }

        // writeln!(out, "ip, it: {} {}", ip, it).ok();
        // writeln!(out, "p, t: {} {}", p, t).ok();

        // My mind is fucking blow by this, finally it could AC
        for i in 0..s {
            let mut r = t + p;

            if a[i] > b[i] {
                if i + 1 < ip {
                    r -= a[i];
                    r -= b[(ip - 1).min(it - 1)];
                } else {
                    r -= a[ip - 1];
                    r -= b[(it - 1).min(i)];
                }
            } else if i + 1 < it {
                r -= b[i];
                r -= a[(it - 1).min(ip - 1)];
            } else {
                r -= b[it - 1];
                r -= a[(ip - 1).min(i)];
            }
            write!(out, "{} ", r).ok();
        }
        writeln!(out).ok();
    }
}
