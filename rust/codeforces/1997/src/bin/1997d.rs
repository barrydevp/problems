// https://codeforces.com/contest/1997/problem/D

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

fn dfs(tree: &Vec<Vec<usize>>, nodes: &Vec<i32>, n: usize) -> i32 {
    let mut k = i32::MAX;

    for &v in &tree[n] {
        let c = dfs(tree, nodes, v);
        if c > nodes[n] {
            k = k.min((c + nodes[n]) / 2);
        } else {
            k = k.min(c);
        }
        // println!("{} {} {}", n, v, k);
    }

    if k == i32::MAX {
        nodes[n]
    } else {
        k
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();
        // let mut v = vec![0; n];
        // for i in 0..n {
        //     v[i] = scan.next::<usize>();
        // }
        let v = scan.next_vec::<i32>(n);
        let mut g = vec![vec![]; n];
        for i in 0..n - 1 {
            let v = scan.next::<usize>() - 1;
            g[v].push(i + 1);
        }

        // let ans = dfs(&g, &v, 0);
        let mut c = i32::MAX;

        for &i in &g[0] {
            c = c.min(dfs(&g, &v, i));
            // println!("{} {}", i, c);
        }

        writeln!(out, "{}", c + v[0]).ok();
    }
}
