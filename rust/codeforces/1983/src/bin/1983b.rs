// https://codeforces.com/contest/1983/problem/B

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
        let (n, m) = (scan.next::<usize>(), scan.next::<usize>());
        let mut a = vec![vec![0; m]; n];
        let mut b = vec![vec![0; m]; n];
        (0..n).for_each(|i| {
            let c = scan.next_chars();
            (0..m).for_each(|j| {
                a[i][j] = (c[j] as u8 - b'0') as i32;
            });
        });
        (0..n).for_each(|i| {
            let c = scan.next_chars();
            (0..m).for_each(|j| {
                b[i][j] = (c[j] as u8 - b'0') as i32;
            });
        });
        let mut ans = true;
        (0..n).for_each(|i| {
            let mut sum = 0;
            (0..m).for_each(|j| {
                sum += a[i][j] - b[i][j];
            });
            ans &= sum % 3 == 0;
        });

        if ans {
            (0..m).for_each(|j| {
                let mut sum = 0;
                (0..n).for_each(|i| {
                    sum += a[i][j] - b[i][j];
                });
                ans &= sum % 3 == 0;
            });
        }
        writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
    }
}
