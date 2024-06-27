// https://codeforces.com/contest/1989/problem/C

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
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
        let a = scan.next_vec::<i32>(n);
        // let b = scan.next_vec::<i32>(n);

        let (mut f, mut s) = (0, 0);
        let (mut inc, mut dec) = (0, 0);

        for i in 0..n {
            let b = scan.next::<i32>();
            if a[i] != b {
                if a[i] > b {
                    f += a[i];
                } else {
                    s += b;
                }
            } else if a[i] > 0 {
                inc += 1;
            } else if a[i] < 0 {
                dec += 1;
            }
        }

        while inc > 0 {
            if f < s {
                f += 1;
            } else {
                s += 1;
            }
            inc -= 1;
        }
        while dec > 0 {
            if f < s {
                s -= 1;
            } else {
                f -= 1;
            }
            dec -= 1;
        }

        writeln!(out, "{}", s.min(f)).ok();
    }
}
