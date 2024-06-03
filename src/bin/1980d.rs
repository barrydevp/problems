// https://codeforces.com/contest/1980/problem/D

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

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();

        // let a = (0..n).map(|_| scan.next::<usize>());
        let mut a = vec![0; n];
        // let mut b = vec![0; n - 1];
        // let mut dp = vec![(0, 1); n];
        a[0] = scan.next::<usize>();
        for i in 1..n {
            let v = scan.next::<usize>();
            a[i] = v;
            // b[i - 1] = gcd(a[i], a[i - 1]);
        }
        // dp[1] = (1, gcd(a[0], a[1]));
        //
        // for i in 2..n {
        //     let b = gcd(a[i], a[i - 1]);
        //     dp[i] = dp[i - 1];
        //     if b >= dp[i - 1].1 {
        //         dp[i] = (dp[i - 1].0 + 1, b);
        //     } else if i > 2 {
        //         let c = gcd(a[i - 1], a[i - 3]);
        //         if c >= dp[i - 2].1 && b >= c {
        //             dp[i - 2].1 = c;
        //             dp[i] = (dp[i - 2].0 + 1, b);
        //         }
        //     }
        // }

        // println!("{:?}", dp);

        // println!("{:?}", b);
        //
        let mut bp = vec![0; n - 1];
        let mut rem = false;
        let mut last = gcd(a[0], a[1]);
        let mut ok = true;
        bp[0] = last;
        for i in 1..n - 1 {
            let b = gcd(a[i], a[i + 1]);
            if b < last {
                if rem {
                    // println!("why? {} {}", b, last);
                    ok = false;
                    break;
                }
                if i == n - 2 {
                    break;
                }
                // println!("first! {} {}", b, last);
                let d = if i > 1 { gcd(a[i], a[i - 2]) } else { 1 };
                let e = if i > 2 { gcd(a[i - 3], a[i - 2]) } else { 1 };
                let c = gcd(a[i - 1], a[i + 1]);
                let f = if i > 1 { gcd(a[i - 1], a[i - 2]) } else { 1 };
                let g = gcd(a[i], a[i + 2]);

                if c < f && (d < e || b < d) && g < last {
                    // println!("why? {} {} {} {} {} {} {}", c, f, d, e, b, g, last);
                    ok = false;
                    break;
                }

                if b >= d && d >= e {
                    if (b < c || c < f) && (b < g || g < last) {
                        last = b;
                    } else if c <= g || g < last {
                        last = c;
                    } else {
                        last = g;
                        a[i + 1] = a[i];
                    }
                } else if c >= f && (c <= g || g < last) {
                    last = c;
                } else {
                    last = g;
                    a[i + 1] = a[i];
                }

                // if b >= d && (b < c || c < f) && d >= e {
                //     last = b;
                // } else if g >= last && g < c {
                // } else {
                //     last = c;
                // }
                rem = true;
            } else {
                last = b;
            }

            // println!("{:?}", bp);
            bp[i] = last;
        }
        // println!("{:?}", bp);

        // writeln!(out, "{}", if (dp[n - 1].0 >= n - 2) { "YES" } else { "NO" }).ok();
        writeln!(out, "{}", if ok { "YES" } else { "NO" }).ok();
    }
}
