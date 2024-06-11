// https://codeforces.com/contest/1985/problem/C

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
    for t in 0..nt {
        let n = scan.next::<usize>();

        let mut a = vec![0; n + 1];

        for i in 1..=n {
            let x = scan.next::<usize>() as u64;
            a[i] = a[i - 1] + x;
        }

        let mut ans = if a[1] == 0 { 1 } else { 0 };
        let mut last = 1;
        for i in 1..=n {
            let x = a[i] - a[i - 1];
            if i > last && x == a[i - 1] {
                // println!("a {}", i);
                ans += 1;
                last = i;
            }
            let mut l = if i + 1 > last { i + 1 } else { last + 1 };
            let mut r = n;

            while l < r {
                let m = l + (r - l) / 2;
                if a[m] >= 2 * x {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            // println!("{} {} {}", l, r, x);
            while l <= n && a[l] == 2 * x {
                ans += 1;
                last = l;
                l += 1;
            }
        }

        // if nt == 10000 {
        //     if t == 510 {
        //         writeln!(out, "{}", ans).ok();
        //         for i in 1..=n {
        //             writeln!(out, "{} ", a[i] - a[i - 1]).ok();
        //         }
        //         writeln!(out, "{:?}", a).ok();
        //     }
        // } else {
            writeln!(out, "{}", ans).ok();
        // }
    }
}
