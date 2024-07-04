// https://oj.vnoi.info/problem/bones

#[macro_use]
extern crate dmoj;

fn main() {
    let (s1, s2, s3) = scan!(usize, usize, usize);

    let mut dp = vec![0; s1 + s2 + s3 + 1];

    for i in 1..=s1 {
        for j in 1..=s2 {
            for k in 1..=s3 {
                dp[i + j + k] += 1;
            }
        }
    }

    let mut max = 0;
    for i in (1..=s1 + s2 + s3).rev() {
        if dp[i] >= dp[max] {
            max = i;
        }
    }

    println!("{}", max);
}
