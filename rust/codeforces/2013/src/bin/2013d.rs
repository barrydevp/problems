// https://codeforces.com/contest/2013/problem/D

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
use std::ops::Div;

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
        let mut a = vec![0; n + 1];
        for i in (1..=n).rev() {
            a[i] = scan.next::<u64>();
        }
        let mut suff = vec![0; n + 1];
        for i in 1..=n {
            suff[i] = suff[i - 1] + a[i];
        }
        let mut stack = vec![(1, a[1])];

        // println!("{:?}", a);
        for i in 2..=n {
            if stack.last().unwrap().1 > a[i] {
                stack.push((i, a[i]));
            } else {
                let mut cur_min = stack.pop().unwrap();
                let mut cur_i = cur_min.0;
                let mut cur = a[i];
                // println!("{:?} --", stack);
                loop {
                    if cur < cur_min.1 {
                        stack.push(cur_min);
                        break;
                    }

                    // println!("{:?} --", stack);
                    let val = suff[i] - suff[cur_min.0 - 1];
                    cur = val.div((i - cur_min.0 + 1) as u64);
                    cur_i = cur_min.0;
                    // println!("1: {:?} {:?}", cur_min.1, cur);

                    if stack.is_empty() {
                        break;
                    }
                    cur_min = stack.pop().unwrap();
                }
                // println!("{:?} {:?}", cur_min, stack);
                stack.push((cur_i, cur));
            }
        }

        // println!("{:?}", suff);
        // println!("{:?}", stack);
        let min = stack.last().unwrap().1;
        let max = if stack.len() < 2 {
            min + if suff[n] % (n as u64) != 0 { 1 } else { 0 }
        } else {
            let i1 = stack[1].0 - 1;
            let i2 = stack[0].0 - 1;
            // println!(
            //     "{:?} {:?} {:?}",
            //     i1,
            //     i2,
            //     (suff[i1] - suff[i2]) / ((i1 - i2) as u64)
            // );
            (suff[i1] - suff[i2]).div_ceil((i1 - i2) as u64)
        };
        // println!("{:?}", suff);
        // println!("{:?}", stack);
        // println!("{:?} {:?}", min, max);
        writeln!(out, "{}", max - min).ok();
    }
}
