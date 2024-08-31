// https://codeforces.com/contest/2007/problem/A

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
    // let mut is_prime = vec![true; 1001];
    //
    // for i in 2..=1000 {
    //     if is_prime[i] {
    //         for j in (2 * i..=1000).step_by(i) {
    //             is_prime[j] = false;
    //         }
    //     }
    // }

    for _ in 0..nt {
        let mut l = scan.next::<usize>();
        let r = scan.next::<usize>();

        if l % 2 == 0 {
            l += 1;
        }

        let mut ans = (r - l + 1) / 4;
        if (r - l + 1) % 4 == 3 && r % 2 == 1 {
            ans += 1;
        }

        writeln!(out, "{}", ans).ok();
    }
}
