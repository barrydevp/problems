// https://codeforces.com/contest/1971/problem/G

#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap};
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
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
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();
        let mut m = HashMap::<i32, BinaryHeap<i32>>::new();
        let mut a = vec![0; n];
        (0..n).for_each(|i| {
            let v = scan.next::<i32>();
            a[i] = v;
            let e = m.entry(v >> 2).or_default();
            e.push(-v);
        });

        (0..n).for_each(|i| {
            let v = a[i] >> 2;
            write!(out, "{} ", -m.get_mut(&v).unwrap().pop().unwrap()).ok();
        });
        writeln!(out).ok();
    }
}
