// https://codeforces.com/contest/1971/problem/F

#[allow(unused_imports)]
use std::cmp::{max, min};
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
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let r = scan.next::<u64>();
        let mut ans = 0;
        for x in 0..=r {
            let y2 = ((r) * (r) - x * x) as u64;
            let mut le = 1;
            let mut ri = y2;
            while le < ri {
                let m = le + (ri - le) / 2;
                if m * m < y2 {
                    le = m + 1;
                } else {
                    ri = m;
                }
            }

            let mut y = ri;
            let c = ((r + 1) * (r + 1) - x * x) as u64;
            // println!("[{},{}, {}]", x, y, y2);
            while y * y < c {
                ans += 1;
                y += 1;
            }
        }
        writeln!(out, "{}", ans * 4 - 4).ok();
    }
}
