// https://codeforces.com/contest/1986/problem/E

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
        let (n, k) = (scan.next::<usize>(), scan.next::<u32>());

        let mut a = vec![0; n];
        let mut m = HashMap::new();

        (0..n).for_each(|i| {
            a[i] = scan.next::<u32>();
            m.entry(a[i] % k).or_insert(BinaryHeap::new()).push(a[i]);
        });

        let mut ans = 0;
        let mut ok = true;
        // println!("{:?}", m);
        for (_, mut v) in m {
            if v.len() % 2 == 1 {
                if !ok {
                    ans = -1;
                    break;
                }
                ok = false;
                if v.len() > 1 {
                    let mut l1 = 0;
                    let mut l2 = 0;
                    let mut i = 1;
                    let mut a = v.pop().unwrap();
                    while !v.is_empty() {
                        let b = v.pop().unwrap();
                        i += 1;
                        let c = (a - b) / k;
                        if i % 2 == 0 {
                            l2 += c;
                        } else {
                            l1 = (l2).min(l1 + c);
                        }
                        a = b;
                    }
                    ans += l1 as i32;
                }
            } else {
                while !v.is_empty() {
                    let a = v.pop().unwrap();
                    let b = v.pop().unwrap();
                    ans += ((a - b) / k) as i32;
                }
            }
        }

        writeln!(out, "{}", ans).ok();
    }
}
