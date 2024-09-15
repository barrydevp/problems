// https://codeforces.com/contest/2005/problem/A

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
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for _ in 0..nt {
        let n = scan.next::<usize>();

        let mut s = vec!['a'; n];

        if n > 5 {
            let rep = n / 5;
            let mut cnt = [rep; 5];
            for i in 0..n % 5 {
                cnt[i] += 1;
            }
            let mut idx = 0;
            // println!("{:?}", cnt);
            for i in 0..5 {
                while cnt[i] > 0 {
                    s[idx] = vowels[i];
                    idx += 1;
                    cnt[i] -= 1;
                }
            }
            writeln!(out, "{}", s.iter().collect::<String>()).ok();
        } else {
            writeln!(out, "{}", vowels[0..n].iter().collect::<String>()).ok();
        }
    }
}
