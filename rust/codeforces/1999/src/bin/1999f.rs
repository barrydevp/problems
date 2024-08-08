// https://codeforces.com/contest/1999/problem/F

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

const MOD: i64 = 1000000007;

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x1, y1) = gcd(b, a % b);
        (d, y1, x1 - y1 * (a / b))
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();

    let mut f = vec![0_i64; 2 * 100_000 + 1];
    f[0] = 1;
    for i in 1..2 * 100_000 + 1 {
        f[i] = f[i - 1] * (i as i64) % MOD;
    }

    for _ in 0..nt {
        let n = scan.next::<usize>();
        let k = scan.next::<usize>();

        let mut ones = 0;
        let mut zeros = 0;
        for _ in 0..n {
            let c = scan.next::<usize>();
            if c == 1 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        let d = (k + 1) / 2;

        // dCn = n! / (n-k)!k!

        // println!("{} - {}", ones, d);
        let mut ans = 0;
        for i in d..=min(ones, k) {
            if k - i > zeros {
                continue;
            }
            let (_, x1, _) = gcd(f[ones - i] * f[i] % MOD, MOD);
            let x1 = if x1 < 0 { x1 + MOD } else { x1 };
            let (_, x2, _) = gcd(f[zeros - (k - i)] * f[k - i] % MOD, MOD);
            let x2 = if x2 < 0 { x2 + MOD } else { x2 };
            let c = (f[ones] * x1 % MOD) * (f[zeros] * x2 % MOD) % MOD;
            ans = (ans + c) % MOD;
        }

        writeln!(out, "{}", ans).ok();
    }
}
