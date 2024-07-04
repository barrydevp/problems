// https://codeforces.com/contest/1979/problem/D

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
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let (n, k) = (scan.next::<usize>(), scan.next::<usize>());

        let mut s: Vec<char> = scan.next::<String>().chars().collect();

        let mut b = s[0];
        let mut c = 1;
        let mut p: i32 = 0;
        for i in 1..n {
            if c == k && s[i] == b {
                println!("{} {}, here", s[i], s[0]);
                p = i as i32;
                break;
            } else if c != k && s[i] != b {
                p = i as i32;
                break;
            } else if c == k && s[i] != b {
                c = 0;
                b = s[i];
            }

            c += 1;
        }
        println!("{} {}", p, c);

        let mut ans = k as i32;
        if p != 0 {
            for i in (1..n).rev() {
                if s[i] == b {
                    c += 1;
                    if c > k {
                        ans = -1;
                        break;
                    }
                } else {
                    if c == k {
                        ans = p;
                    } else {
                        ans = -1;
                    }
                    break;
                }
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
