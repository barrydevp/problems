// https://codeforces.com/contest/2002/problem/C

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

fn diff(x: (usize, usize), y: (usize, usize)) -> usize {
    let dx = y.0.abs_diff(x.0);
    let dy = y.1.abs_diff(x.1);
    dx * dx + dy * dy
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();

        let mut c = vec![(0, 0); n];
        for i in 0..n {
            c[i].0 = scan.next::<usize>();
            c[i].1 = scan.next::<usize>();
        }

        let s = (scan.next::<usize>(), scan.next::<usize>());
        let t = (scan.next::<usize>(), scan.next::<usize>());

        let d = diff(s, t);

        let mut ans = true;
        for i in 0..n {
            if diff(c[i], t) <= d {
                ans = false;
                break;
            }
        }

        writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
    }
}
