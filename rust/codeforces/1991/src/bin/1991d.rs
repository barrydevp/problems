// https://codeforces.com/contest/1991/problem/D

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
    let pre = [1, 2, 2, 3, 3, 4];

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();

        if n < 6 {
            writeln!(out, "{}", pre[n - 1]).ok();
            for i in 0..n {
                write!(out, "{} ", pre[i]).ok();
            }
            writeln!(out).ok();
        } else {
            // we can have a graph with [1,3,4,6], they are all prime when XOR each other
            // so the mimimum number of colors is 4
            // when the number of color is 4, need no more than 4 colors for them
            // (the easiest way to color is to use 4 colors in a cycle)
            // because each same color will be in the form of 4k + 0 or 4k + 1 or 4k + 2 or 4k + 3
            // so because of XOR operation we will get 00 for the last two bits (as last two bits
            // are the same)
            // and because it is 4k, then their XOR is multiple of 4 and is not a prime number
            // eg: (4a + 1) ^ (4b + 1) = 4(a ^ b) // 4
            writeln!(out, "{}", 4).ok();
            for i in 1..=n {
                write!(out, "{} ", i % 4 + 1).ok();
            }
            writeln!(out).ok();
        }
    }
}
