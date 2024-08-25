// https://codeforces.com/contest/2003/problem/C

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

        let mut s = scan.next_chars();

        let mut c = [(0, 'a'); 26];

        for i in 0..n {
            c[s[i] as usize - 'a' as usize].0 += 1;
            c[s[i] as usize - 'a' as usize].1 = s[i];
        }

        c.sort();
        let mut i = 0;
        let mut k = 0;
        while k < n {
            while c[i].0 == 0 {
                i = (i + 1) % 26;
            }
            write!(out, "{}", c[i].1).ok();
            c[i].0 -= 1;
            i = (i + 1) % 26;
            k += 1;
        }

        writeln!(out).ok();
    }
}
