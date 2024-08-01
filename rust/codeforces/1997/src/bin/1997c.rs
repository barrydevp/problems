// https://codeforces.com/contest/1997/problem/C

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
        let mut a = scan.next_chars();

        let mut ans = 0;
        // let mut s = vec![];
        //
        // for i in 0..n {
        //     if a[i] == '_' {
        //         if s.len() == 0 {
        //             a[i] = '(';
        //             s.push(i);
        //         } else {
        //             a[i] = ')';
        //             let v = s.pop().unwrap();
        //             ans += i - v;
        //         }
        //     } else {
        //         if a[i] == '(' {
        //             s.push(i);
        //         } else {
        //             let v = s.pop().unwrap();
        //             ans += i - v;
        //         }
        //     }
        // }

        ans = (n * (n + 1) / 2) as i32;
        ans = -ans;
        let mut c = 0;
        for i in 0..n {
            if a[i] == '(' {
                c += 1;
            } else if a[i] == ')' {
                c -= 1;
                ans += (i as i32 + 1) * 2;
            } else {
                if c > 0 {
                    ans += (i as i32 + 1) * 2;
                    c -= 1;
                } else {
                    c += 1;
                }
            }
        }
        // println!("{:?}", a);

        writeln!(out, "{}", ans).ok();
    }
}
