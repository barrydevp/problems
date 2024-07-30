// https://codeforces.com/contest/1997/problem/A

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
        // let n = scan.next::<usize>();
        let a = scan.next_chars();

        let mut max = (1, 1);
        let mut c = 1;
        for i in 1..a.len() {
            if a[i] == a[i - 1] {
                c += 1;
            } else {
                if max.1 < c {
                    max = (i - c / 2, c);
                }
                c = 1;
            }
        }
        if max.1 < c {
            max = (a.len() - c / 2, c);
        }
        // println!("{:?}", max);

        for i in 0..max.0 {
            write!(out, "{}", a[i]);
        }
        let mut c = 'a';
        if max.1 == 1 && max.0 != a.len() {
            // c = if a[max.0 - 1] != 'a' && a[max.0] != 'a' {
            //     'a'
            // } else if a[max.0 - 1] != 'b' && a[max.0] != 'b' {
            //     'b'
            // } else {
            //     'c'
            // };
            c = (b'a'
                + ((a[max.0] as u8).min(a[max.0 - 1] as u8) - b'a'
                    + if (a[max.0] as u8).abs_diff(a[max.0 - 1] as u8) == 1 {
                        1
                    } else {
                        0
                    }
                    + 1)
                    % 26) as char;
        } else {
            c = (b'a' + (a[max.0 - 1] as u8 - b'a' + 1) % 26) as char;
        }
        write!(out, "{}", c);

        for i in max.0..a.len() {
            write!(out, "{}", a[i]);
        }

        writeln!(out).ok();
    }
}
