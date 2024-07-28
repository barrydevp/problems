// https://codeforces.com/contest/1991/problem/c

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
        let mut a = scan.next_vec::<usize>(n);

        let mut x = vec![];
        let mut ans = 1;
        for _ in 0..40 {
            let mut ma = a[0];
            let mut mi = a[0];
            for i in 1..n {
                if a[i] > ma {
                    ma = a[i];
                } else if a[i] < mi {
                    mi = a[i];
                }
            }

            if ma == 0 {
                break;
            }

            let v = (ma + mi) / 2;
            x.push(v);
            ans += 1;
            if v * 2 != (ma + mi) {
                ans = -1;
                break;
            }
            for i in 0..n {
                a[i] = a[i].abs_diff(v);
            }
        }

        if ans == 41 || ans == -1 {
            writeln!(out, "{}", -1).ok();
        } else {
            writeln!(out, "{}", ans - 1).ok();
            x.iter().for_each(|&v| {
                write!(out, "{} ", v).ok();
            });
            writeln!(out).ok();
        }

        // if j == -1 {
        //     writeln!(out, "{}", ans).ok();
        //     for i in 0..ans {
        //         write!(out, "{} ", x[i]).ok();
        //     }
        //     writeln!(out).ok();
        // } else {
        //     writeln!(out, "{}", -1).ok();
        // }
    }
}
