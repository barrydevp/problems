// https://codeforces.com/contest/1998/problem/A

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

        let x = scan.next::<i32>();
        let y = scan.next::<i32>();
        let k = scan.next::<i32>();

        let n = if k % 2 == 0 { k - 2 } else { k - 1 };
        let mut j = 1;
        for _ in 0..n / 2 {
            if j == x.abs() * k && j == y.abs() * k {
                j += 1;
            }
            writeln!(out, "{} {}", j, j).ok();
            writeln!(out, "{} {}", -j, -j).ok();
            j += 1;
        }

        if k - n == 1 {
            writeln!(out, "{} {}", x * k, y * k).ok();
        } else if x == 0 && y == 0 {
            writeln!(out, "{} {}", j, j).ok();
            writeln!(out, "{} {}", -j, -j).ok();
        } else {
            writeln!(out, "{} {}", x * k, 0).ok();
            writeln!(out, "{} {}", 0, y * k).ok();
        }

        // writeln!(out, "{}", n).ok();
    }
}
