// https://codeforces.com/contest/2001/problem/D

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
use std::{cmp::Reverse, collections::BinaryHeap};

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

        let a = scan.next_vec::<usize>(n);

        // let mut m = HashMap::new();

        let mut u = vec![false; n + 1];
        let mut o = vec![];
        for i in (0..n).rev() {
            // m.insert(a[i], i);
            if !u[a[i]] {
                u[a[i]] = true;
                o.push(i);
            }
        }
        // let mut o = m.values().collect::<Vec<_>>();
        // if nt != 1 || n == 300000 || a[0] == 300000 || a[1] == 1 {
        //     o.sort();
        //     // writeln!(out, "{}", n).ok();
        //     // for i in 0..n {
        //     //     write!(out, "{} ", a[i]).ok();
        //     // }
        //     // writeln!(out).ok();
        //     // continue;
        // }

        // println!("{:?}", m);
        // let mut u = HashMap::new();
        let mut u = vec![false; n + 1];
        let mut p = vec![0; o.len()];
        // println!("{:?}", o);

        let mut i = 0;
        let mut j = 0;
        let mut l = 0;
        let mut c = 0;
        let no = o.len();
        let mut odd_max = BinaryHeap::new();
        let mut even_min = BinaryHeap::new();
        while i < no {
            let mut lidx = o[no - i - 1];

            while u[a[lidx]] && i < (no - 1) {
                i += 1;
                lidx = o[no - i - 1];
            }

            while l <= lidx {
                if !u[a[l]] {
                    odd_max.push((a[l], l));
                    even_min.push(Reverse((a[l], l)));
                }
                l += 1;
            }

            // println!("{:?} {:?}", odd_max, even_min);

            while (c % 2 == 0 && !odd_max.is_empty()) || (c % 2 == 1 && !even_min.is_empty()) {
                let mut eadd = None;
                if (c + 1) % 2 == 1 {
                    let mut e = odd_max.pop();
                    while let Some(v) = e {
                        if u[v.0] || v.1 < j {
                            e = odd_max.pop();
                            continue;
                        }

                        if let Some(nv) = odd_max.peek() {
                            if v.0 == nv.0 && nv.1 >= j {
                                e = odd_max.pop();
                                continue;
                            }
                        }
                        break;
                    }
                    eadd = e;
                } else {
                    let mut e = even_min.pop();
                    while let Some(Reverse(v)) = e {
                        if !u[v.0] && (v.1 >= j && v.1 <= lidx) {
                            break;
                        }
                        e = even_min.pop();
                    }
                    if let Some(Reverse(v)) = e {
                        eadd = Some(v);
                    }
                }
                // println!("eadd: {:?}", eadd);
                if let Some(v) = eadd {
                    if !u[v.0] {
                        u[v.0] = true;
                        p[c] = v.0;
                        j = v.1 + 1;
                        c += 1;
                        if v.0 == a[lidx] {
                            break;
                        }
                    }
                }
            }

            // println!("{:?} {:?}", odd_max, even_min);

            i += 1;
            // println!("{}-{}-{}", i, j, c);
        }
        // while i < no {
        //     let lidx = o[no - i - 1];
        //     // if u.contains_key(&a[*o[i]]) {
        //     if u[a[lidx]] {
        //         i += 1;
        //         continue;
        //     }
        //     let mut v = (0, 0);
        //     while j <= lidx {
        //         // if !u.contains_key(&a[j]) {
        //         if !u[a[j]] {
        //             if c % 2 == 0 {
        //                 if v.0 < a[j] {
        //                     v = (a[j], j);
        //                 }
        //             } else if v.0 == 0 || v.0 > a[j] {
        //                 v = (a[j], j);
        //             }
        //         }
        //         j += 1;
        //     }
        //     if v.0 != 0 {
        //         u[v.0] = true;
        //         p[c] = v.0;
        //         c += 1;
        //         j = v.1 + 1;
        //     } else {
        //         // j = lidx + 1;
        //         i += 1;
        //     }
        //     // println!("{}-{}-{}", i, j, c);
        // }

        // let mut dup = vec![];
        //
        // for k in m.keys() {
        //     let e = m.get(k).unwrap();
        //     if (*e).len() > 1 {
        //         dup.push(*k);
        //     }
        // }
        //
        // dup.sort();
        // for i in (0..dup.len()).rev() {
        //     let oc = m.get_mut(&dup[i]).unwrap();
        //     let ocn = oc.len();
        //     for i in 0..ocn {
        //         if oc[i] % 2 == 1 {
        //             *oc = vec![oc[i]];
        //             break;
        //         }
        //     }
        //     if (*oc).len() > 1 {
        //         *oc = vec![oc[oc.len() - 1]];
        //     }
        // }
        //
        // // println!("{:?}", m);
        //
        // writeln!(out, "{}", m.keys().len()).ok();
        //
        // for i in 0..n {
        //     let e = m.get_mut(&a[i]).unwrap();
        //     if (*e).len() > 0 && (*e)[0] == i + 1 {
        //         write!(out, "{} ", a[i]).ok();
        //         *e = vec![];
        //     }
        // }
        //
        // writeln!(out).ok();

        writeln!(out, "{}", p.len()).ok();
        p.iter().for_each(|x| {
            write!(out, "{} ", x).ok();
        });
        writeln!(out).ok();
    }
}
