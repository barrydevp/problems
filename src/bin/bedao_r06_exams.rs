// https://oj.vnoi.info/problem/bedao_r06_exams

#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);

    let (mut a, mut b) = (vec![0; n + 1], vec![0; n + 1]);
    for i in 1..=n {
        let x = scan!(usize);

        a[i] = x;
        b[i] = b[i - 1] + x;
    }

    let mut dp1 = vec![0; n + 1];
    dp1[2] = a[1] * a[2];
    let mut dp2 = vec![0; n + 1];

    for i in 3..=n {
        dp1[i] = (dp1[i - 1] + b[i - 1] * a[i] % 1_000_000_007) % 1_000_000_007;
        dp2[i] = (dp2[i - 1] + dp1[i - 1] * a[i] % 1_000_000_007) % 1_000_000_007;
    }

    println!("{}", dp2[n]);
}
