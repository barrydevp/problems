// https://codeforces.com/contest/2004/problem/D

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
        let n = scan.next::<usize>();
        let q = scan.next::<usize>();

        let a = scan
            .next_line()
            .split_whitespace()
            .map(|x| {
                x.chars()
                    .map(|c| match c {
                        'B' => 1,
                        'G' => 2,
                        'R' => 4,
                        'Y' => 8,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, x| acc | x)
            })
            .collect::<Vec<usize>>();

        let mut c = vec![vec![]; 16];

        for i in 0..n {
            c[a[i]].push(i + 1);
        }

        let mut pre = vec![vec![0; n + 2]; 16];
        let mut suf = vec![vec![0; n + 2]; 16];

        for k in 1..16 {
            for i in 1..=n {
                pre[k][i] = if a[i - 1] == k { i } else { pre[k][i - 1] };
            }
            for i in (1..=n).rev() {
                suf[k][i] = if a[i - 1] == k { i } else { suf[k][i + 1] };
            }
        }

        for _ in 0..q {
            let x = scan.next::<usize>();
            let y = scan.next::<usize>();
            let (x, y) = if x < y { (x, y) } else { (y, x) };
            let mut ans = i32::MAX;
            if a[x - 1] & a[y - 1] != 0 {
                ans = x.abs_diff(y) as i32;
            } else {
                for i in 1..16 {
                    if a[x - 1] == i || a[y - 1] == i {
                        continue;
                    }

                    if pre[i][y] != 0 {
                        ans = min(ans, (y.abs_diff(pre[i][y]) + x.abs_diff(pre[i][y])) as i32);
                    }

                    if suf[i][x] != 0 {
                        ans = min(ans, (x.abs_diff(suf[i][x]) + y.abs_diff(suf[i][x])) as i32);
                    }
                }
            }
            if ans == i32::MAX {
                ans = -1;
            }
            writeln!(out, "{}", ans).ok();
        }

        // writeln!(out, "{}", n).ok();
    }
}
