// https://codeforces.com/contest/2000/problem/C

#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::HashMap;
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
        let a = scan.next_vec::<i32>(n);
        let m = scan.next::<usize>();
        for _ in 0..m {
            let cc = scan.next_chars();
            let mut o = [-1; 26];
            let mut h = HashMap::new();
            let mut ans = true;
            if cc.len() != n {
                ans = false;
            } else {
                for (i, &c) in cc.iter().enumerate() {
                    let ci = ((c as u8) - b'a') as usize;
                    // println!("{:?}", o);
                    if o[ci] != -1 && a[o[ci] as usize] != a[i] {
                        ans = false;
                        break;
                    }
                    o[ci] = i as i32;

                    if let Some(&j) = h.get(&a[i]) {
                        if cc[i] != cc[j] {
                            ans = false;
                            break;
                        }
                    }
                    h.insert(a[i], i);
                }
            }
            writeln!(out, "{}", if ans { "YES" } else { "NO" }).ok();
        }
    }
}
