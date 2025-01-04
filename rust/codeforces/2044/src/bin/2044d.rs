// https://codeforces.com/contest/2044/problem/D

#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::HashMap;
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
        let a = scan.next_vec::<usize>(n);

        let mut h = HashMap::new();
        for i in 0..n {
            h.entry(a[i]).or_insert(i);
        }

        let mut unused = 1;

        let mut b = vec![0; n];

        for i in 0..n {
            if let Some(&j) = h.get(&a[i]) {
                if j < i {
                    while h.contains_key(&unused) {
                        unused += 1;
                    }
                    b[i] = unused;
                    unused += 1;
                } else {
                    b[i] = a[i];
                }
            }
        }

        b.iter().for_each(|x| {
            write!(out, "{} ", x).ok();
        });
        writeln!(out).ok();
    }
}
