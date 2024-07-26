// https://codeforces.com/contest/1996/problem/E

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
        // let n = scan.next::<usize>();
        let s = scan.next_chars();
        let n = s.len();

        let mut a = vec![0_i32; n + 1];
        for i in 0..n {
            if s[i] == '0' {
                a[i + 1] = a[i] - 1;
            } else {
                a[i + 1] = a[i] + 1;
            }
        }

        // because we have a[i] = sum of [1,i]
        // in order to have the same '1' and '0' for a range [l,r], we need a[r-1] == a[l]
        // because: a[l] = a[r-1] + v (the sum of all char from [r,l] as '0' = -1 and '1' = 1),
        // and this sum must equal to 0 as we need number of '0' equal '1' => a[l] = a[r-1] + 0
        // Call dp[k] as the number of possible 'r' that have the sum equal to k.
        // Thus, we iterate from 1->n, for each i, we have k = a[i] and the number of possible 'r'
        // from i -> n is n-i+1
        let mut dp = vec![0; 2 * n + 1];
        for i in 0..=n {
            let nn = n as i32;
            let val = (a[i] + nn) as usize;
            dp[val] += n - i + 1;
        }

        let mut ans = 0;
        for i in 1..=n {
            let nn = n as i32;
            let val = (a[i - 1] + nn) as usize;
            dp[val] -= n - (i - 1) + 1;
            ans = (ans + dp[val] * (i) % 1_000_000_007) % 1_000_000_007;
        }

        writeln!(out, "{}", ans).ok();
    }
}
