// https://codeforces.com/contest/1990/problem/C

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
        // let mut m = HashMap::<usize, usize>::new();
        let mut m = vec![0; n + 1];

        let mut b = vec![0; n + 1];
        let mut sum = 0;
        let mut maxx = 0;
        (0..n).for_each(|i| {
            let x = scan.next::<usize>();
            // let e = m.entry(x);
            // *e.or_default() += 1;
            // if (*m.get(&x).unwrap()) > 1 {
            m[x] += 1;
            if m[x] > 1 {
                if x > maxx {
                    maxx = x;
                    b[i] = x;
                } else {
                    b[i] = b[i - 1];
                }
            } else if i > 0 {
                b[i] = b[i - 1];
            } else {
                b[i] = 0;
            }
            sum += x;
        });

        let mut dp = vec![0; n];
        dp[0] = b[0];
        let mut prefix = 0;
        for i in 1..n {
            if b[i] != b[i - 1] {
                dp[i] = dp[i - 1];
            } else {
                dp[i] = b[i];
            }
            prefix += dp[i];
            sum += b[i];
        }

        for i in (0..n).rev() {
            sum += prefix;
            prefix -= dp[i];
        }

        writeln!(out, "{}", sum).ok();
    }
}
