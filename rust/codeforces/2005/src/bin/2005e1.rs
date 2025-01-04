// https://codeforces.com/contest/2005/problem/E1

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

fn search(b: &Vec<Vec<usize>>, sx: usize, sy: usize, val: usize) -> (usize, usize) {
    let n = b.len();
    let m = b[0].len();
    let mut i = n;
    let mut j = m;
    for i in (sx..n).rev() {
        for j in (sy..m).rev() {
            if b[i][j] == val {
                return (i, j);
            }
        }
    }
    (n, m)
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let l = scan.next::<usize>();
        let n = scan.next::<usize>();
        let m = scan.next::<usize>();
        let a = scan.next_vec::<usize>(l);
        let mut b = vec![];

        for _ in 0..n {
            b.push(scan.next_vec::<usize>(m));
        }

        // let mut loc = vec![vec![]; 8];
        // for i in 0..n {
        //     for j in 0..m {
        //         loc[b[i][j]].push(((n - (i + 1)) * (m - (j + 1)), i, j));
        //     }
        // }
        // for i in 0..8 {
        //     loc[i].sort();
        // }
        // // println!("{:?}", loc);
        //
        // let mut end = false;
        // let mut i = 0;
        // let mut sn = 0;
        // let mut sm = 0;
        // while !end {
        //     let ca = a[i];
        //     let mut found = loc[ca].len();
        //     for j in 0..loc[ca].len() {
        //         let (d, x, y) = loc[ca][j];
        //         if x >= sn && y >= sm {
        //             sn = x + 1;
        //             sm = y + 1;
        //             found = j;
        //             break;
        //         }
        //     }
        //     // not found
        //     if found == loc[ca].len() {
        //         // println!("notfound 1: {} {}", sn, sm);
        //         i += 1;
        //         end = true;
        //         break;
        //     }
        //     i += 1;
        //     if i == l {
        //         i -= 1;
        //         end = true;
        //         break;
        //     }
        //     // println!("{}: {}", i, a.len());
        //
        //     let nca = a[i];
        //     let mut nfound = loc[nca].len();
        //     let tmp_sn = loc[ca][loc[ca].len() - 1].1 + 1;
        //     let tmp_sm = loc[ca][loc[ca].len() - 1].2 + 1;
        //     for j in 0..loc[nca].len() {
        //         let (d, x, y) = loc[nca][j];
        //         if x >= tmp_sn && y >= tmp_sm {
        //             nfound = j;
        //             break;
        //         }
        //     }
        //     // not found
        //     if nfound == loc[nca].len() {
        //         // println!("notfound 2: {} {}", sn, sm);
        //         i -= 1;
        //         end = true;
        //         break;
        //     }
        //     // println!("{}: {} {}", i, sn, sm);
        // }

        let mut ans = 0;
        let mut sx = 0;
        let mut sy = 0;
        for i in 0..l {
            ans = i;
            let aa = a[i];
            let (x, y) = search(&b, sx, sy, aa);
            if x == n && y == m {
                ans += 1;
                break;
            }
            sx = x + 1;
            sy = y + 1;
            if ans == l - 1 {
                ans += 1;
            }
        }

        writeln!(out, "{}", if ans % 2 == 0 { "T" } else { "N" }).ok();
    }
}
