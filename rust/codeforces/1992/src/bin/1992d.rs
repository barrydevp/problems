// https://codeforces.com/contest/1992/problem/D

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
        let m = scan.next::<usize>();
        let k = scan.next::<usize>();

        let a = scan.next_chars();

        let mut dp = vec![false; n + 2];
        dp[0] = true;

        let mut i = 0;
        let mut l = k;

        while i < n + 1 {
            let mut newi = i + 1;
            if i == 0 {
                for j in (1..=m).rev() {
                    if i + j > n + 1 {
                        continue;
                    }
                    if i + j == n + 1 {
                        dp[n + 1] = true;
                        newi = n + 1;
                    } else if a[i + j - 1] == 'L' {
                        dp[i + j] = true;
                        newi = i + j;
                        break;
                    } else if a[i + j - 1] == 'W' {
                        dp[i + j] = true;
                        newi = newi.max(i + j);
                    }
                }
            } else if dp[i] {
                if a[i - 1] == 'L' {
                    for j in (1..=m).rev() {
                        if i + j > n + 1 {
                            continue;
                        }
                        if i + j == n + 1 {
                            dp[n + 1] = dp[i] && true;
                            newi = n + 1;
                        } else if a[i + j - 1] == 'L' {
                            dp[i + j] = dp[i] && true;
                            newi = i + j;
                            break;
                        } else if a[i + j - 1] == 'W' {
                            dp[i + j] = dp[i] && true;
                            newi = newi.max(i + j);
                        }
                    }
                } else {
                    let mut j = 0;
                    while l > 0 {
                        j += 1;
                        l -= 1;
                        if i + j > n + 1 {
                            break;
                        }
                        if i + j == n + 1 {
                            dp[n + 1] = dp[i] && true;
                            newi = n + 1;
                            break;
                        }
                        if a[i + j - 1] == 'C' {
                            newi = n + 1;
                            break;
                        }
                        if a[i + j - 1] == 'L' {
                            dp[i + j] = dp[i] && true;
                            newi = i + j;
                            break;
                        } else if a[i + j - 1] == 'W' {
                            dp[i + j] = dp[i] && true;
                            newi = newi.max(i + j);
                        }
                    }
                }
            }
            i = newi;
        }

        writeln!(out, "{}", if dp[n + 1] { "YES" } else { "NO" }).ok();
    }
}
