// https://codeforces.com/contest/2008/problem/F

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
        let mut sum = 0;
        let a = scan.next_vec::<i64>(n);

        // for i in 0..n {
        //     sum = (sum + a[i]) % MOD;
        // }

        let mut tu = 0i64;
        for i in 0..n {
            // tu = (tu + a[i] * (sum - a[i]) % MOD) % MOD;
            tu = (tu + sum * a[i]) % MOD;
            sum = (sum + a[i]) % MOD;
        }
        // tu /= 2;

        let (_, x, _) = gcd(f[n - 2] * 2 % MOD, MOD);
        let x = if x < 0 { x + MOD } else { x };
        let (_, imau, _) = gcd(f[n] * x % MOD, MOD);
        // let (_, imau, _) = gcd((n as i64 * (n - 1) as i64) / 2 % MOD, MOD);
        let imau = if imau < 0 { imau + MOD } else { imau };
        let ans = ((tu % MOD) * imau) % MOD;

        writeln!(out, "{}", ans).ok();
    }
}
