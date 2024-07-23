// https://codeforces.com/contest/1995/problem/B2

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
    for t in 0..nt {
        let n = scan.next::<usize>();
        let m = scan.next::<u64>();

        let mut a = vec![(0_u64, 0_u64); n];
        for i in 0..n {
            a[i].0 = scan.next::<u64>();
        }
        for i in 0..n {
            a[i].1 = scan.next::<u64>();
        }
        // if nt == 10000 && t == 2951 {
        //     writeln!(out, "{}-{}-{}-{}", a[0].0, a[1].0, a[0].1, a[1].1).ok();
        //     // writeln!(out, "{}-{}", n, m).ok();
        //     // writeln!(out, "{}", 999).ok();
        // }

        a.sort_by(|a, b| a.0.cmp(&b.0));

        let mut ans = 0;

        for i in (0..n) {
            let c = a[i].0 * a[i].1;
            if c == m {
                ans = max(ans, m);
                break;
            }
            if i < (n - 1) && a[i].0 == a[i + 1].0 - 1 {
                // let x = m / a[i].0;
                // if x <= a[i].1 {
                //     ans = max(ans, x * a[i].0 + (m - x * a[i].0).min(x).min(a[i + 1].1));
                //     // println!("1:{}", ans);
                // } else {
                //     let y = (m - a[i].1 * a[i].0) / a[i + 1].0;
                //     if y <= a[i + 1].1 {
                //         let c1 = a[i].1 * a[i].0 + y * a[i + 1].0;
                //         ans = max(ans, c1 + (m - c1).min(a[i].1).min(a[i + 1].1 - y));
                //         // println!("2:{}", ans);
                //     } else {
                //         ans = max(ans, a[i].1 * a[i].0 + a[i + 1].1 * a[i + 1].0);
                //         // println!("3:{}", ans);
                //     }
                // }

                let x = (m / a[i].0).min(a[i].1);
                let y = ((m - x * a[i].0) / a[i + 1].0).min(a[i + 1].1);
                // now sell as much flower a[i] and buy as much flower a[i+1] as possible
                // because we can replace flow a[i] by a[i+1] to increase cost by 1
                let s = x.min(a[i + 1].1 - y).min(m - x * a[i].0 - y * a[i + 1].0);
                ans = max(ans, x * a[i].0 + y * a[i + 1].0 + s);
            } else {
                ans = max(ans, min(c, m / a[i].0 * a[i].0));
                // println!("4:{}", ans);
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
