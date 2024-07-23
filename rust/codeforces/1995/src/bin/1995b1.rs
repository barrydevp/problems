// https://codeforces.com/contest/1995/problem/B

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
        let m = scan.next::<u64>();

        let mut a = scan.next_vec::<u64>(n);
        a.sort();

        let mut ans = 0;
        // let mut id = (n - 1) as i32;
        // // println!("{:?}", a);
        // while id >= 0 {
        //     let i = id as usize;
        //     let x = a[i];
        //     let mut ix = a.binary_search(&x).unwrap();
        //     while ix > 0 && a[ix] == x {
        //         ix -= 1;
        //     }
        //     if a[ix] != x {
        //         ix += 1;
        //     }
        //     let c = ((i - ix + 1) as u64) * a[i];
        //     if c >= m {
        //         ans = max(ans, m);
        //         break;
        //     } else if ix > 0 && a[ix - 1] == x - 1 {
        //         let l = m - c;
        //         let x1 = a[ix - 1];
        //         let mut ix1 = a.binary_search(&x1).unwrap();
        //         while ix1 > 0 && a[ix1] == x1 {
        //             ix1 -= 1;
        //         }
        //         if a[ix1] != x1 {
        //             ix1 += 1;
        //         }
        //         let c1 = (ix - 1 - ix1 + 1) as u64 * x1;
        //         if c1 >= l {
        //             ans = max(ans, m);
        //             break;
        //         } else {
        //             ans = max(ans, c + c1);
        //         }
        //         // println!("{} {} {} {}", i, ix, ix1, ans);
        //     } else {
        //         ans = max(ans, c);
        //     }
        //     id = ix as i32 - 1;
        // }

        let mut r = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += a[i];
            while r < n && (sum > m || a[i] > a[r] + 1) {
                sum -= a[r];
                r += 1;
            }
            ans = max(ans, sum);
        }

        writeln!(out, "{}", ans).ok();
    }
}
