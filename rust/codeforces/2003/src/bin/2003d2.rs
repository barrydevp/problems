// https://codeforces.com/contest/2003/problem/D2

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
    let mut vis = vec![false; 200_003];
    let mut adj = vec![vec![]; 200_003];
    for _ in 0..nt {
        let n = scan.next::<usize>();
        let m = scan.next::<usize>() as i64;

        let mut max_fst = 0;
        let mut max_snd = 0;
        let mut pairs = vec![(0, 0); n];
        for i in 0..n {
            let l = scan.next::<usize>();
            let mut a = vec![0; l];

            for j in 0..l {
                let v = scan.next::<usize>();
                a[j] = v;
                if v <= 200_000 {
                    vis[v] = true;
                }
            }

            let mut nn = 0;
            let mut p = (0, 0);
            for j in 0..200_003 {
                if vis[j] {
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

            pairs[i] = p;
            max_fst = max(max_fst, p.0 as i64);
            max_snd = max(max_snd, p.1 as i64);

            for j in 0..l {
                if a[j] <= 200_000 {
                    vis[a[j]] = false;
                }
            }
        }

        for i in 0..=max_snd {
            adj[i as usize].clear();
        }

        for i in 0..n {
            adj[pairs[i].0 as usize].push(pairs[i].1 as usize);
        }

        let mut mpp = vec![0i64; (max_snd as usize) + 1];
        for i in (0..=max_snd as usize).rev() {
            mpp[i] = max(max_fst, i as i64);
            for j in 0..adj[i].len() {
                mpp[i] = mpp[i].max(mpp[adj[i][j]]);
            }
            if adj[i].len() > 1 {
                max_fst = max(max_fst, mpp[i]);
            }
        }
        let mut ans = 0;
        for i in 0..=min(max_snd, m) as usize {
            ans += max(max_fst, mpp[i]);
        }

        if max_snd < m {
            ans += (m + max_snd + 1) * (m - max_snd) / 2;
        }

        writeln!(out, "{}", ans).ok();
    }
}
