// https://codeforces.com/contest/1994/problem/A

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
        let n = scan.next::<usize>();
        let m = scan.next::<usize>();

        if n == 1 {
            if m == n {
                scan.next::<usize>();
                write!(out, "{}", -1).ok();
            } else {
                let mut a = vec![0; m];
                for i in 0..m {
                    a[(i + 1) % m] = scan.next::<usize>();
                }
                a.iter().for_each(|x| {
                    write!(out, "{} ", x).ok();
                });
            }
            writeln!(out).ok();
        } else {
            let mut a = vec![vec![0; m]; n];
            (0..n).for_each(|i| {
                (0..m).for_each(|j| {
                    a[(i + 1) % n][j] = scan.next::<usize>();
                });
            });

            a.iter().for_each(|c| {
                c.iter().for_each(|x| {
                    write!(out, "{} ", x).ok();
                });
                writeln!(out).ok();
            });
        }
    }
}
