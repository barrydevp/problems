// https://codeforces.com/contest/2008/problem/B

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

        let s = scan.next_chars();
        let mut ans = true;
        let mut r = 0;
        let mut c = 1;
        while c * c < n {
            c += 1;
        }
        if c * c != n {
            ans = false;
        } else {
            let r = c;
            // println!("r: {} c: {}", r, c);
            for j in 0..c {
                let idx = j;
                if s[idx] == '0' {
                    ans = false;
                    break;
                }
            }
            for i in 1..r - 1 {
                for j in 0..c {
                    let idx = (i) * c + j;
                    if (j == 0 || j == c - 1) {
                        if s[idx] == '0' {
                            ans = false;
                            // println!("flase 1, idx: {}", idx);
                            break;
                        }
                    } else if s[idx] == '1' {
                        // println!("flase 2, idx: {}", idx);
                        ans = false;
                        break;
                    }
                }
            }

            for j in 0..c {
                let idx = (r - 1) * c + j;
                if s[idx] == '0' {
                    ans = false;
                    break;
                }
            }
        }

        writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
    }
}
