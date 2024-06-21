// https://codeforces.com/contest/1971/problem/E

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
        let (n, k, q) = (
            scan.next::<usize>(),
            scan.next::<usize>(),
            scan.next::<usize>(),
        );
        let a: Vec<usize> = (0..k + 1)
            .map(|i| if i == 0 { 0 } else { scan.next::<usize>() })
            .collect();
        let mut b = vec![0; k + 1];
        (0..k).for_each(|i| {
            b[i + 1] = scan.next::<usize>();
        });

        (0..q).for_each(|_| {
            let d = scan.next::<usize>();
            let mut ans = 0;
            if d != 0 {
                let mut l = 0;
                let mut r = k;
                while l < r {
                    let m = l + (r - l) / 2;

                    if a[m] < d {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }

                // println!("{:?}", a);
                // println!("{:?}", b);
                // println!("{}", l);
                if a[l] == d {
                    ans = b[l];
                } else {
                    ans = b[l - 1] + (d - a[l - 1]) * (b[l] - b[l - 1]) / (a[l] - a[l - 1]);
                }
            }
            write!(out, "{} ", ans).ok();
        });
        writeln!(out).ok();
    }
}
