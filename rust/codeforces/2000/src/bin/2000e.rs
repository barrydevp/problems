// https://codeforces.com/contest/2000/problem/E

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
        let n = scan.next::<i32>();
        let m = scan.next::<i32>();
        let k = scan.next::<i32>();
        let w = scan.next::<usize>();
        let mut a = scan.next_vec::<i64>(w);

        let mut s = vec![0; (m * n) as usize];

        for i in 0..n {
            for j in 0..m {
                let lx = max(0, i - k + 1);
                let rx = min(i, n - k);
                let ly = max(0, j - k + 1);
                let ry = min(j, m - k);
                s[(i * m + j) as usize] = (rx - lx + 1) * (ry - ly + 1);
            }
        }

        a.sort();
        s.sort();
        // println!("{:?}", s);
        let mut ans = 0i64;
        let mut j = (m * n) as usize;
        for i in (0..w).rev() {
            ans += a[i] * s[j - 1] as i64;
            j -= 1;
        }

        writeln!(out, "{}", ans).ok();
    }
}
