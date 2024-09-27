// https://codeforces.com/contest/2014/problem/D

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
        let d = scan.next::<usize>();
        let k = scan.next::<usize>();

        // let mut tree = SegTree::new(n, 0, std::cmp::max);
        let mut a = vec![(0, 0); k];
        // prefix[i] = number of jobs has been started until i
        let mut prefix = vec![0; n + 2];
        // suffix[i] = number of jobs has been ended until i
        let mut suffix = vec![0; n + 2];
        for i in 0..k {
            a[i] = (scan.next::<usize>(), scan.next::<usize>());
            prefix[a[i].0] += 1;
            suffix[a[i].1] += 1;
            // a[i] = (a[i].1, a[i].0);
            // let v = tree.query(a[i].0, a[i].1);
            // println!("l: {}, r: {}, j: {}", a[i].0, a[i].1, v);
            // tree.insert_range(a[i].0, a[i].1, v + 1);
        }
        // a.sort();
        // tree.insert_range(a[0].1, a[0].0, 1);
        // for i in 1..k {
        //     if a[i].0 > a[i - 1].0 {
        //         // overlapped
        //         if a[i - 1].1 <= a[i].0 && a[i - 1].0 >= a[i].1 {
        //             let v = tree.query(a[i].1, a[i].0);
        //             tree.insert_range(a[i].1, a[i].0, v + 1);
        //         } else {
        //             let v = tree.query(a[i].1, a[i].0);
        //             tree.insert_range(a[i].1, a[i].0, v + 1);
        //         }
        //     } else {
        //         let v = tree.query(a[i].1, a[i].0);
        //         tree.insert_range(a[i].1, a[i].0, v + 1);
        //     }
        // }
        for i in 1..=n {
            prefix[i] += prefix[i - 1];
            suffix[i] += suffix[i - 1];
        }
        // let overlap = max(0, min(e1, e2) - max(s1, s2));
        let start_limit = n - d + 1;
        // let mut dp = vec![0; n + 1];
        let mut mother_job = i32::MAX;
        let mut mother_start = 1;
        let mut brother_job = i32::MIN;
        let mut brother_start = 1;
        for i in 1..=start_limit {
            // we have the visit range of [i, i + d - 1]
            // therefore, we take total jobs that has been started until i + d - 1
            // and subtract the total jobs that has been ended before i which is at i - 1
            let v = prefix[i + d - 1] - suffix[i - 1];

            // let v = tree.query(i, i + d - 1);
            // // println!("l: {}, r: {}, j: {}", i, i + d - 1, v);
            if mother_job > v {
                mother_job = v;
                mother_start = i;
            }
            if brother_job < v {
                brother_job = v;
                brother_start = i;
            }
        }
        // println!("mother: {} brother: {}", mother_job, brother_job);

        writeln!(out, "{} {}", brother_start, mother_start).ok();
    }
}
