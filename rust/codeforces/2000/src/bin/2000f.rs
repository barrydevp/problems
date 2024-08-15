// https://codeforces.com/contest/2000/problem/F

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
        let k = scan.next::<usize>();
        let mut a = vec![(0, 0); n];

        for i in 0..n {
            let x = scan.next::<usize>();
            let y = scan.next::<usize>();
            a[i] = if x < y { (x, y) } else { (y, x) };
        }

        let mut v = vec![0; 201];
        let mut dp = vec![vec![10_000_001; k + 1]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            let mut x = a[i - 1].0;
            let mut y = a[i - 1].1;
            let total_points = x + y;

            let mut points = 0;
            while points < total_points {
                points += 1;
                v[points] = v[points - 1]
                    + if x < y {
                        y -= 1;
                        x
                    } else {
                        x -= 1;
                        y
                    };
            }
            // println!("v- {:?}", v);
            // println!("p- {:?}", points);

            dp[i][0] = 0;
            for j in (1..=k).rev() {
                for l in 0..=min(j, points) {
                    dp[i][j] = min(dp[i][j], dp[i - 1][j - l] + v[l] as i32);
                }
            }
        }

        // println!("{:?}", dp);
        let mut ans = dp[n][k];
        if ans == 10_000_001 {
            ans = -1;
        }

        writeln!(out, "{}", ans).ok();
    }
}
