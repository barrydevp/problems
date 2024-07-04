// https://codeforces.com/contest/1975/problem/A

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
        let n = scan.next::<usize>();
        let a = (0..n).map(|_| scan.next::<usize>()).collect::<Vec<usize>>();

        let mut b = 0;
        for i in 1..n {
            if a[i] < a[i - 1] {
                b += 1;
            }
        }
        writeln!(
            out,
            "{}",
            if b == 0 || (b == 1 && a[n - 1] <= a[0]) {
                "Yes"
            } else {
                "No"
            }
        )
        .ok();
    }
}
