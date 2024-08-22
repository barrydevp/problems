// https://codeforces.com/contest/2001/problem/C

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

        let mut edges = vec![vec![]; n + 1];

        for i in (2..=n).rev() {
            let mut a = 1;
            let mut b = i;
            while a != b {
                writeln!(out, "? {} {}", a, b).ok();
                out.flush().ok();
                let m = scan.next::<usize>();

                if m == a {
                    b = m;
                } else if m == b {
                    b = a;
                } else {
                    a = m;
                }
            }

            edges[a].push(i);
        }

        write!(out, "!").ok();
        for i in 1..=n {
            for v in &edges[i] {
                write!(out, " {} {}", i, v).ok();
            }
        }
        writeln!(out).ok();
        out.flush().ok();
    }
}
