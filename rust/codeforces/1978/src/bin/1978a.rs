// https://codeforces.com/contest/1978/problem/A

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

    // let nt = scan.next::<usize>();
    // for _ in 0..nt {
    //     let n = scan.next::<usize>();
    //
    //     let (mut a, mut b) = (0, 0);
    //
    //     (0..n).for_each(|_| {
    //         let v = scan.next::<usize>();
    //
    //         if a > b {
    //             b = v;
    //         } else {
    //             a = v;
    //         }
    //     });
    //
    //     writeln!(out, "{}", a + b).ok();
    // }

    let x = scan.next::<u32>();
    let y = scan.next::<u32>();

    if x * x < y {
        println!("x*x < y");
    } else {
        println!("x*x >= y");
    }
}
