// https://codeforces.com/contest/2013/problem/C

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

        let mut pwd_suffix = String::new();

        // first, try to find the suffix of the string
        while pwd_suffix.len() < n {
            writeln!(out, "? {}0", pwd_suffix).ok();
            out.flush().ok();

            if scan.next::<usize>() == 1 {
                pwd_suffix.push('0');
                continue;
            }

            writeln!(out, "? {}1", pwd_suffix).ok();
            out.flush().ok();
            if scan.next::<usize>() == 0 {
                // we found the suffix
                break;
            }
            pwd_suffix.push('1');
        }

        let n = n - pwd_suffix.len();
        let mut pwd_prefix = vec!['0'; n];
        // ok now we have the suffix, let's find the prefix
        for i in (0..n).rev() {
            write!(out, "? ").ok();
            for j in i..n {
                write!(out, "{}", pwd_prefix[j]).ok();
            }
            writeln!(out, "{}", pwd_suffix).ok();
            out.flush().ok();
            pwd_prefix[i] = if scan.next::<usize>() == 1 { '0' } else { '1' };
        }

        writeln!(
            out,
            "! {}{}",
            pwd_prefix.iter().collect::<String>(),
            pwd_suffix
        )
        .ok();
        out.flush().ok();
    }
}
