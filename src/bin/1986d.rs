// https://codeforces.com/contest/1986/problem/D

#[allow(unused_imports)]
use std::cmp::{max, min};
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
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
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
        let s: Vec<_> = scan
            .next_line()
            .trim()
            .chars()
            .map(|x| x.to_string().parse::<i32>().expect("Failed parse"))
            .collect();

        let mut first_zero = -1;
        let mut l1 = s[0] * 10 + s[1];
        let mut l2 = s[0];

        for i in 2..n {
            if s[i] == 0 && first_zero < 0 {
                first_zero = i as i32;
            }
            (l1, l2) = (
                (l1 + s[i])
                    .min(l2 + s[i - 1] * 10 + s[i])
                    .min(l1 * s[i])
                    .min(l2 * (s[i - 1] * 10 + s[i])),
                (l2 + s[i - 1]).min(l2 * s[i - 1]),
            );
        }
        let mut ans = l1;

        // println!("{} {}", ans, first_zero);
        // if first_zero > -1 && n > 2 {
        //     let f = first_zero as usize;
        //     if n == 3 && f == 1 {
        //         ans = ans.min(s[0] * (s[2]));
        //     } else {
        //         ans = 0;
        //     }
        // }
        writeln!(out, "{}", ans).ok();
    }
}
