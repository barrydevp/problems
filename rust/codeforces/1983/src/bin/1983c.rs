// https://codeforces.com/contest/1983/problem/C

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

fn bin_search(a: &[usize], b: &[usize], c: &[usize], t: usize) -> Option<(bool, usize, usize)> {
    let mut r1 = 1;
    while a[r1] < t {
        r1 += 1;
    }
    let n = a.len() - 1;
    let mut m = r1 + 1;
    while m < n {
        // println!("m, r1: {} {}", m, r1);
        // println!("m, r1: {} {}", b[m], b[r1]);
        // println!("{:?}", b);
        if (b[m] - b[r1]) < t {
            m += 1;
        } else if c[n] - c[m] >= t {
            return Some((true, r1, m));
        } else {
            break;
        }
    }

    let mut m = r1 + 1;
    while m < n {
        if (c[m] - c[r1]) < t {
            m += 1;
        } else if b[n] - b[m] >= t {
            return Some((false, r1, m));
        } else {
            break;
        }
    }
    None
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();
        // let a = scan.next_vec::<usize>(n);
        // let b = scan.next_vec::<usize>(n);
        // let c = scan.next_vec::<usize>(n);

        // let tot = a.iter().sum::<usize>();
        let mut tot = 0;
        let mut a = vec![0; n + 1];
        (1..=n).for_each(|i| {
            let v = scan.next::<usize>();
            tot += v;
            a[i] = a[i - 1] + v;
        });
        let mut b = vec![0; n + 1];
        (1..=n).for_each(|i| {
            let v = scan.next::<usize>();
            b[i] = b[i - 1] + v;
        });
        let mut c = vec![0; n + 1];
        (1..=n).for_each(|i| {
            let v = scan.next::<usize>();
            c[i] = c[i - 1] + v;
        });

        let target = tot.div_ceil(3);
        let mut ans = false;
        if let Some((inorder, r1, r2)) = bin_search(&a, &b, &c, target) {
            if inorder {
                writeln!(out, "{} {} {} {} {} {}", 1, r1, r1 + 1, r2, r2 + 1, n).ok();
            } else {
                writeln!(out, "{} {} {} {} {} {}", 1, r1, r2 + 1, n, r1 + 1, r2).ok();
            }
            ans = true;
        } else if let Some((inorder, r1, r2)) = bin_search(&b, &a, &c, target) {
            if inorder {
                writeln!(out, "{} {} {} {} {} {}", r1 + 1, r2, 1, r1, r2 + 1, n).ok();
            } else {
                writeln!(out, "{} {} {} {} {} {}", r2 + 1, n, 1, r1, r1 + 1, r2).ok();
            }
            ans = true;
        } else if let Some((inorder, r1, r2)) = bin_search(&c, &a, &b, target) {
            if inorder {
                writeln!(out, "{} {} {} {} {} {}", r1 + 1, r2, r2 + 1, n, 1, r1).ok();
            } else {
                writeln!(out, "{} {} {} {} {} {}", r2 + 1, n, r1 + 1, r2, 1, r1).ok();
            }
            ans = true;
        }

        if !ans {
            writeln!(out, "{}", -1).ok();
        }
    }
}
