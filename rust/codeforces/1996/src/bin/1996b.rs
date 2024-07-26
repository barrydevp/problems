// https://codeforces.com/contest/1996/problem/B

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
        let k = scan.next::<usize>();

        let mut a: Vec<Vec<char>> = vec![];
        (0..n).for_each(|_| {
            a.push(scan.next_chars());
        });

        (0..n).step_by(k).for_each(|i| {
            (0..n).step_by(k).for_each(|j| {
                write!(out, "{}", a[i][j]).ok();
            });
            writeln!(out).ok();
        });
        // if k != 1 {
        //     let mut i = 0;
        //     while i < n {
        //         let mut j = 0;
        //         while j < n {
        //             write!(out, "{}", a[i][j]).ok();
        //             j += k;
        //         }
        //         writeln!(out).ok();
        //         i += k;
        //     }
        // } else {
        //     a.iter().for_each(|x| {
        //         x.iter().for_each(|y| {
        //             write!(out, "{}", y).ok();
        //         });
        //         writeln!(out).ok();
        //     });
        // }

        // writeln!(out, "{}", n).ok();
    }
}
