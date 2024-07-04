// https://codeforces.com/contest/1980/problem/C

#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
};

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
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();
        let a: Vec<usize> = (0..n).map(|_| scan.next::<usize>()).collect();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut mapb: HashMap<usize, bool> = HashMap::new();
        // let b: Vec<usize> = (0..n).map(|_| scan.next::<usize>()).collect();
        let mut needed = 0;
        for i in 0..n {
            let v = scan.next::<usize>();
            mapb.entry(v).or_insert(true);

            if v != a[i] {
                (*map.entry(v).or_insert(0)) += 1;
                needed += 1;
            }
        }

        let m = scan.next::<usize>();
        // let d: Vec<usize> = (0..n).map(|_| scan.next::<usize>()).collect();
        let mut ire = false;
        for _ in 0..m {
            let v = scan.next::<usize>();

            if let Some(en) = map.get_mut(&v) {
                if *en > 0 {
                    (*en) -= 1;
                    needed -= 1;
                }
            }

            if mapb.contains_key(&v) {
                ire = false;
            } else {
                ire = true;
            }
        }

        if ire || needed > 0 {
            writeln!(out, "{}", "NO").ok();
        } else {
            writeln!(out, "{}", "YES").ok();
        }

        // writeln!(out, "{} {}", n, m).ok();
    }
}
