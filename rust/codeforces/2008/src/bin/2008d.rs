// https://codeforces.com/contest/2008/problem/D

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

fn cal(p: &Vec<usize>, s: &Vec<char>, dp: &mut Vec<i32>, i: usize) -> i32 {
    if dp[i] != -1 {
        return dp[i];
    }
    dp[i] = 0;
    let j = p[i] - 1;
    dp[i] += cal(p, s, dp, j) + if s[i] == '0' { 1 } else { 0 };

    dp[i]
}

fn build(p: &Vec<usize>, head: &mut Vec<i32>, i: usize) -> i32 {
    if head[i] != -1 {
        return head[i];
    }

    head[i] = i as i32;
    head[i] = build(p, head, p[i] - 1);
    head[i]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();
        let mut p = scan.next_vec::<usize>(n);
        let s = scan.next_chars();

        let mut dp = vec![-1; n + 1];
        let mut head = vec![-1; n + 1];
        for i in 0..n {
            build(&p, &mut head, i);
        }
        // println!("{:?}", head);

        // for i in 0..n {
        //     if dp[head[i]] != -1 {
        //         dp[i] = dp[head[i]];
        //         continue;
        //     }
        //     dp[i] = 0;
        //     let mut j = p[i] - 1;
        //     let mut blacks = if s[i] == '0' { 1 } else { 0 };
        //     while dp[j] != -1 {
        //         head[j] = i;
        //         if s[j] == '0' {
        //             blacks += 1;
        //         }
        //         if j == p[j] - 1 {
        //             break;
        //         }
        //         j = p[j] - 1;
        //     }
        //     dp[i] = blacks;
        // }
        for i in 0..n {
            if dp[head[i] as usize] == -1 {
                cal(&p, &s, &mut dp, i);
            }
            let mut ans = dp[head[i] as usize];
            // if head[i] as usize != i {
            //     ans -= if s[i] == '0' { 1 } else { 0 };
            // }
            write!(out, "{} ", ans).ok();
        }
        writeln!(out).ok();
    }
}
