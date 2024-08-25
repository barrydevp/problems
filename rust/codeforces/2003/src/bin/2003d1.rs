// https://codeforces.com/contest/2003/problem/D1

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
        let m = scan.next::<usize>();

        let mut c = vec![false; 200_003];
        let mut mp = (0, 0);
        for i in 0..n {
            let l = scan.next::<usize>();

            let mut a = vec![0; l];

            for j in 0..l {
                let v = scan.next::<usize>();
                a[j] = v;
                if v <= 200_000 {
                    c[v] = true;
                }
            }

            let mut nn = 0;
            let mut p = (0, 0);
            for j in 0..200_003 {
                if c[j] {
                    continue;
                }
                if nn == 0 {
                    p.0 = j;
                    nn += 1;
                } else {
                    p.1 = j;
                    break;
                }
            }
            if mp.1 < p.1 {
                mp = p;
            }
            for j in 0..l {
                if a[j] <= 200_000 {
                    c[a[j]] = false;
                }
            }
        }

        let mut ans = (mp.1 as i64) * (min(mp.1 + 1, m + 1) as i64);
        // println!("mp: {:?}", mp);
        // println!("ans: {:?}", ans);
        if mp.1 < m {
            ans -= mp.1 as i64;
            ans += ((m + mp.1) as i64) * ((m - mp.1 + 1) as i64) / 2;
        }

        writeln!(out, "{}", ans).ok();
    }
}
