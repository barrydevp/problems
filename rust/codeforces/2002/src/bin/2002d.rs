// https://codeforces.com/contest/2002/problem/D1+D2

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

fn dfs(u: usize, m: &mut usize, adj: &Vec<Vec<usize>>, tour: &mut [Vec<usize>; 2]) {
    tour[0][u] = *m;
    *m += 1;
    for &v in adj[u].iter() {
        dfs(v, m, adj, tour);
    }
    tour[1][u] = *m;
}

fn is_ancestor(u: usize, v: usize, tour: &[Vec<usize>; 2]) -> bool {
    tour[0][u] <= tour[0][v] && tour[1][v] <= tour[1][u]
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();

    for _ in 0..nt {
        let n = scan.next::<usize>();
        let q = scan.next::<usize>();

        let mut parent = vec![0; n + 1];
        let mut adj = vec![vec![]; n + 1];
        for i in 2..=n {
            parent[i] = scan.next::<usize>();
            adj[parent[i]].push(i);
        }

        let mut tour = [vec![0; n + 1], vec![0; n + 1]];
        let mut m = 1;

        dfs(1, &mut m, &adj, &mut tour);
        // println!("{:?}", tour);

        let mut p = vec![0; n + 1];
        for i in 1..=n {
            p[i] = scan.next::<usize>();
        }

        let mut correct_pairs = 0_i32;

        for i in 1..n {
            correct_pairs += is_ancestor(parent[p[i + 1]], p[i], &tour) as i32;
        }

        for _ in 0..q {
            let x = scan.next::<usize>();
            let y = scan.next::<usize>();
            let x_v = p[y];
            let y_v = p[x];

            let mut update = |x: usize, x_v: usize| {
                if x > 1 {
                    correct_pairs -= is_ancestor(parent[p[x]], p[x - 1], &tour) as i32;
                }
                if x < n {
                    correct_pairs -= is_ancestor(parent[p[x + 1]], p[x], &tour) as i32;
                }
                p[x] = x_v;
                if x > 1 {
                    correct_pairs += is_ancestor(parent[p[x]], p[x - 1], &tour) as i32;
                }
                if x < n {
                    correct_pairs += is_ancestor(parent[p[x + 1]], p[x], &tour) as i32;
                }
            };
            update(x, x_v);
            update(y, y_v);

            writeln!(
                out,
                "{}",
                if correct_pairs == (n - 1) as i32 {
                    "Yes"
                } else {
                    "No"
                }
            )
            .ok();
        }
    }
}
