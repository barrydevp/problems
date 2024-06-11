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

// another use
// fn main() {
//     let mut scan = Scanner::default();
//     let out = &mut BufWriter::new(stdout());
//
//     let nt = scan.next::<usize>();
//     for _ in 0..nt {
//         let n = scan.next::<usize>();
//
//         let mut ans = 0;
//         let mut max = 0;
//         let mut sum = 0;
//
//         for _ in 0..n {
//             let x = scan.next::<usize>();
//             max = max.max(x as u64);
//             sum += x as u64;
//             if sum == max * 2 {
//                 ans += 1;
//             }
//         }
//
//         writeln!(out, "{}", ans).ok();
//     }
// }

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

        writeln!(out, "{}", ans).ok();
    }
}
