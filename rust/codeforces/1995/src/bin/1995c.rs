// https://codeforces.com/contest/1995/problem/C

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
        let a = scan.next_vec::<u64>(n);

        let mut ans = 0_i64;

        // for i in 1..n {
        //     if a[i] == 1 && a[i - 1] > 1 {}
        // }
        for i in 1..n {
            if a[i - 1] > 1 && a[i] == 1 {
                ans = -1;
                break;
            }
        }
        if ans == 0 {
            let mut c = vec![0; n];

            for i in 1..n {
                let (mut x, mut y) = if a[i] > a[i - 1] {
                    (a[i - 1], a[i])
                } else {
                    (a[i], a[i - 1])
                };

                let mut k = 0;

                // while x != 1 && x * x <= y {
                //     x *= x;
                //     k += 1;
                // }

                while y > x && x != 1 {
                    x *= x;
                    k += 1;
                }
                if x > y {
                    k -= 1;
                }

                if a[i] < a[i - 1] {
                    c[i] = c[i - 1] + k;
                    if x != y {
                        c[i] += 1;
                    }
                    // if x < y {
                    //     c[i] += 1;
                    // }
                } else {
                    c[i] = max(c[i - 1] - k, 0);
                }

                // let mut x = a[i - 1];
                // let mut y = a[i];
                // let mut k = 0;
                //
                // while x != 1 && x * x <= y {
                //     x *= x;
                //     k -= 1;
                // }
                // while y != 1 && y < x {
                //     y *= y;
                //     k += 1;
                // }
                // c[i] = max(0, c[i - 1] + k);

                ans += c[i];
            }
            // println!("{:?}", c);
        }

        writeln!(out, "{}", ans).ok();
    }
}
