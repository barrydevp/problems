// https://codeforces.com/contest/1991/problem/E

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

fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<usize>, vertex: usize) -> bool {
    for &adj in &graph[vertex] {
        if color[adj] == 0 {
            color[adj] = 3 - color[vertex];
            if !dfs(graph, color, adj) {
                return false;
            }
        } else if color[adj] == color[vertex] {
            return false;
        }
    }
    true
}

fn is_bipartite(graph: &Vec<Vec<usize>>, color: &mut Vec<usize>) -> bool {
    color[0] = 1;
    dfs(graph, color, 0)
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let nt = scan.next::<usize>();
    for _ in 0..nt {
        let n = scan.next::<usize>();
        let m = scan.next::<usize>();
        let mut graph = vec![vec![]; n];
        let mut color = vec![0; n];

        for _ in 0..m {
            let u = scan.next::<usize>() - 1;
            let v = scan.next::<usize>() - 1;
            graph[u].push(v);
            graph[v].push(u);
        }

        if !is_bipartite(&graph, &mut color) {
            writeln!(out, "Alice").ok();
            out.flush().ok();
            for _ in 0..n {
                writeln!(out, "1 2").ok();
                out.flush().ok();
                scan.next::<usize>();
                scan.next::<usize>();
            }
        } else {
            writeln!(out, "Bob").ok();
            out.flush().ok();
            let mut p1 = vec![];
            let mut p2 = vec![];
            for i in 0..n {
                if color[i] == 1 {
                    p1.push(i + 1);
                } else {
                    p2.push(i + 1);
                }
            }
            // writeln!(out, "{:?}", p1).ok();
            // writeln!(out, "{:?}", p2).ok();
            // out.flush().ok();

            for i in 0..n {
                let c1 = scan.next::<usize>();
                let c2 = scan.next::<usize>();

                if (c1 == 1 || c2 == 1) && p1.len() > 0 {
                    writeln!(out, "{} {}", p1.pop().unwrap(), 1).ok();
                } else if (c1 == 2 || c2 == 2) && p2.len() > 0 {
                    writeln!(out, "{} {}", p2.pop().unwrap(), 2).ok();
                } else if p1.len() > 0 {
                    writeln!(
                        out,
                        "{} {}",
                        p1.pop().unwrap(),
                        if c1 == 2 { c2 } else { c1 }
                    )
                    .ok();
                } else {
                    writeln!(
                        out,
                        "{} {}",
                        p2.pop().unwrap(),
                        if c1 == 1 { c2 } else { c1 }
                    )
                    .ok();
                }

                out.flush().ok();
            }
        }
    }
}
