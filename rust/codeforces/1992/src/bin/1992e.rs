// https://codeforces.com/contest/1992/problem/E

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
        let n = scan.next::<i32>();

        let mut a = vec![];

        if n == 1 {
            for i in 1..10000 {
                a.push((i + 1, i));
            }
        } else {
            let mut nn = 10;
            let mut ni = 1;
            while n / nn > 0 {
                nn *= 10;
                ni += 1;
            }
            nn /= 10;
            // println!("{} {}", nn, ni);

            let mut nnn = 0;
            let mut nii = 0;
            while nnn <= 10000000 {
                nnn = nnn * nn * 10 + n;
                nii += ni;
            }
            // println!("{} {}", nnn, nii);

            for i in 1..=8 {
                let c = nnn;
                for j in 1..=10000 {
                    let aa = j;
                    let b = (aa * (n - 1) - c) + aa;
                    if (aa * ni - nii) == b && b > 0 && b <= min(10000, aa * n) {
                        a.push((aa, b));
                    }
                }
                nnn /= 10;
                nii -= 1;
            }
        }

        writeln!(out, "{}", a.len()).ok();
        for (a, b) in a {
            writeln!(out, "{} {}", a, b).ok();
        }
    }
}
