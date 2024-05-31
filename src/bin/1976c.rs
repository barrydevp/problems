// https://codeforces.com/contest/1976/problem/C

#[allow(unused_imports)]
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nt: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..nt {
        let nm: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (n, m) = (nm[0], nm[1]);

        let a: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let b: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let s = n + m + 1;

        let mut p: i64 = 0;
        let mut ip = 0;
        let mut np = n + 1;
        while np > 0 {
            if a[ip] > b[ip] || np == (s - ip) {
                p += a[ip] as i64;
                np -= 1;
            }
            ip += 1;
        }

        let mut t: i64 = 0;
        let mut it = 0;
        let mut nt = m + 1;
        while nt > 0 {
            if a[it] < b[it] || nt == (s - it) {
                t += b[it] as i64;
                nt -= 1;
            }
            it += 1;
        }

        // My mind is fucking blow by this, finally it could AC
        for i in 0..s {
            let mut r = t + p;

            if a[i] > b[i] {
                if i + 1 < ip {
                    r -= a[i] as i64;
                    r -= b[(ip - 1).min(it - 1)] as i64;
                } else {
                    r -= a[ip - 1] as i64;
                    r -= b[(it - 1).min(i)] as i64;
                }
            } else if i + 1 < it {
                r -= b[i] as i64;
                r -= a[(it - 1).min(ip - 1)] as i64;
            } else {
                r -= b[it - 1] as i64;
                r -= a[(ip - 1).min(i)] as i64;
            }
            print!("{} ", r);
        }
        println!();
    }
}
