// https://codeforces.com/contest/2005/problem/B

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
        let q = scan.next::<usize>();
        let mut b = scan.next_vec::<usize>(m);
        b.sort();

        for i in 0..q {
            let a = scan.next::<usize>();

            let idx = b.binary_search(&a);
            match idx {
                Ok(ii) => {
                    writeln!(out, "0").ok();
                }
                Err(ii) => {
                    if ii == 0 {
                        writeln!(out, "{}", b[ii]-1).ok();
                    } else if ii == m {
                        writeln!(out, "{}", n - b[m - 1]).ok();
                    } else {
                        writeln!(out, "{}", (b[ii] - b[ii - 1]) / 2).ok();
                    }
                }
            }
        }
    }
}
