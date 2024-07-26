// https://codeforces.com/contest/1996/problem/C

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
    let mut aa = vec![[0; 26]; 200_001];
    let mut bb = vec![[0; 26]; 200_001];
    for _ in 0..nt {
        let _ = scan.next::<usize>();
        let q = scan.next::<usize>();

        let a = scan.next_chars();
        for i in 0..26 {
            aa[0][i] = 0;
            bb[0][i] = 0;
        }
        let mut i = 0;
        a.iter().for_each(|c| {
            for j in 0..26 {
                aa[i + 1][j] = aa[i][j];
            }
            let j = *c as usize - 'a' as usize;
            aa[i + 1][j] += 1;
            i += 1;
        });

        let b = scan.next_chars();
        let mut i = 0;
        b.iter().for_each(|c| {
            for j in 0..26 {
                bb[i + 1][j] = bb[i][j];
            }
            let j = *c as usize - 'a' as usize;
            bb[i + 1][j] += 1;
            i += 1;
        });

        // for i in 0..=n {
        //     println!("a: {:?}, b: {:?}", &aa[i], &bb[i]);
        // }
        for _ in 0..q {
            let l = scan.next::<usize>();
            let r = scan.next::<usize>();
            let mut ans = 0;
            for i in 0..26 {
                let x = aa[r][i] - aa[l-1][i];
                let y = bb[r][i] - bb[l-1][i];
                if y > x {
                    ans += y - x;
                }
            }
            writeln!(out, "{}", ans).ok();
        }
    }
}
