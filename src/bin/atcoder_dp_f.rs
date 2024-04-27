// https://oj.vnoi.info/problem/atcoder_dp_f

use dmoj::{print, println};

static mut DP: [[usize; 3001]; 3001] = [[0; 3001]; 3001];

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.chars().collect::<Vec<char>>();
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.chars().collect::<Vec<char>>();

    for i in 0..s.len() {
        for j in 0..t.len() {
            unsafe {
                DP[i + 1][j + 1] = if s[i] == t[j] {
                    DP[i][j] + 1
                } else {
                    DP[i][j + 1].max(DP[i + 1][j])
                }
            }
        }
    }

    let mut r = vec!['a'; unsafe { DP[s.len()][t.len()] }];
    let mut i = s.len();
    let mut j = t.len();
    let mut idx = r.len();
    while idx > 0 {
        if s[i - 1] == t[j - 1] {
            r[idx - 1] = s[i - 1];
            idx -= 1;
            i -= 1;
            j -= 1;
        } else if unsafe { DP[i][j - 1] > DP[i - 1][j] } {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    for c in r {
        print!("{}", c);
    }
    println!();
    // println!("{}", r.iter().collect::<String>());
}
