// https://codeforces.com/contest/1998/problem/B

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

        let mut p = scan.next_vec::<i32>(n);

        let mut q = vec![0; n];
        q[0] = p[n - 1];
        for i in 0..n - 1 {
            q[i + 1] = p[i];
        }
        // if n % 2 == 1 {
        //     q[n - 1] = p[n / 2];
        // }
        //
        // let k = (n - n % 2) / 2;
        // let mut f = true;
        // for i in 0..k {
        //     if f {
        //         q[i] = p[k + i + n % 2];
        //         q[k * 2 - i - 1] = p[k - i - 1];
        //         f = false;
        //     } else {
        //         f = true;
        //         q[i] = p[k - i - 1];
        //         q[k * 2 - i - 1] = p[k + i + n % 2];
        //     }
        // }

        for i in 0..n {
            write!(out, "{} ", q[i]).ok();
        }

        writeln!(out).ok();
    }
}
