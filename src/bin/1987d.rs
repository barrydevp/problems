// https://codeforces.com/contest/1987/problem/D

#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::HashMap;
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
        let mut a = scan.next_vec::<usize>(n);
        a.sort();
        let mut cnt = vec![];
        let mut l = Some(a[0]);
        let mut i = 1;
        while let Some(v) = l {
            let mut len = 1;
            while i < n && v == a[i] {
                len += 1;
                i += 1;
            }
            cnt.push(len);
            if i < n {
                l = Some(a[i]);
            } else {
                l = None;
            }
            i += 1;
        }
        // if nt == 500 && t == 11 {
        // println!("{:?}", a);
        // println!("{:?}", cnt);
        //     continue;
        // }

        let n = cnt.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            dp[i][0] = 0;
            for j in 1..=n {
                dp[i][j] = dp[i - 1][j];
                if dp[i - 1][j - 1] < i32::MAX {
                    let v = dp[i - 1][j - 1] + cnt[i - 1];
                    if v <= (i - j) as i32 {
                        dp[i][j] = dp[i][j].min(v);
                    }
                }
            }
        }

        let mut bobmax = 0;
        for i in (1..=n).rev() {
            if dp[n][i] < i32::MAX {
                bobmax = i;
                break;
            }
        }
        // println!("{:?}", dp);

        writeln!(out, "{}", n - bobmax).ok();
    }
}
