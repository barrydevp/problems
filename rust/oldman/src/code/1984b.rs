// https://codeforces.com/contest/1984/problem/B

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
    (0..nt).for_each(|_| {
        let mut n = scan.next::<u64>();

        let mut ans = true;
        if n % 10 == 9 {
            ans = false;
        } else {
            n /= 10;
            while n > 9 {
                if n % 10 == 0 {
                    ans = false;
                    break;
                }
                n /= 10;
            }
            if n != 1 {
                ans = false;
            }
        }
        writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
    })
}
