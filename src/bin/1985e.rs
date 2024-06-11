// https://codeforces.com/contest/1985/problem/E

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
        let (x, y, z) = (
            scan.next::<usize>() as u64,
            scan.next::<usize>() as u64,
            scan.next::<usize>() as u64,
        );
        let k = scan.next::<u64>();

        let mut ans = 0;
        for i in 1..=x {
            for j in 1..=y {
                let h = k / (i * j);
                if h <= z && h * i * j == k {
                    ans = ans.max((x - i + 1) * (y - j + 1) * (z - h + 1));
                }
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
