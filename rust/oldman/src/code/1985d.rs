// https://codeforces.com/contest/1985/problem/C

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
        let (n, m) = (scan.next::<usize>(), scan.next::<usize>());

        let a: Vec<Vec<char>> = (0..n)
            .map(|_| scan.next::<String>().chars().collect::<Vec<char>>())
            .collect();

        let mut first = (0, 0);

        'outer: for i in 0..n {
            for j in 0..m {
                if a[i][j] == '#' {
                    first = (i, j);
                    break 'outer;
                }
            }
        }

        let mut last = first;
        for i in first.0..n {
            if a[i][first.1] == '#' {
                last.0 = i;
            } else {
                break;
            }
        }

        writeln!(out, "{} {}", (last.0 + first.0) / 2 + 1, first.1 + 1).ok();
    }
}
