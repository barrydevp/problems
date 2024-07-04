// https://codeforces.com/contest/1986/problem/B

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
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }
    fn next_line(&mut self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        input
    }
}

static DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn max_neighbor(i: usize, j: usize, a: &Vec<Vec<i32>>) -> i32 {
    let mut max = -1;
    for (dx, dy) in DIRECTIONS.iter() {
        let x = i as i32 + dx;
        let y = j as i32 + dy;
        if x >= 0 && x < a.len() as i32 && y >= 0 && y < a[0].len() as i32 {
            if a[x as usize][y as usize] >= a[i][j] {
                return -1;
            } else {
                max = max.max(a[x as usize][y as usize]);
            }
        }
    }
    max
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let (n, m) = (scan.next::<usize>(), scan.next::<usize>());

        let mut a = vec![vec![0; m]; n];
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                a[i][j] = scan.next::<i32>();
            });
        });

        // handle
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                let max = max_neighbor(i, j, &a);
                if max > 0 {
                    a[i][j] = max;
                }
            });
        });

        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                write!(out, "{} ", a[i][j]).ok();
            });
            writeln!(out).ok();
        });
    }
}
