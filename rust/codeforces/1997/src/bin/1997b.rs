// https://codeforces.com/contest/1997/problem/B

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
        let a1 = scan.next_chars();
        let a2 = scan.next_chars();

        let mut ans = 0;
        for i in 1..(n - 1) {
            if a1[i] == '.' {
                if a1[i] == a1[i + 1]
                    && a1[i] == a1[i - 1]
                    && a1[i] == a2[i]
                    && a1[i] != a2[i + 1]
                    && a1[i] != a2[i - 1]
                {
                    ans += 1;
                }
            }
            if a2[i] == '.' {
                if a2[i] == a2[i + 1]
                    && a2[i] == a2[i - 1]
                    && a2[i] == a1[i]
                    && a2[i] != a1[i + 1]
                    && a2[i] != a1[i - 1]
                {
                    ans += 1;
                }
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
