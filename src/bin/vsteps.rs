// https://oj.vnoi.info/problem/vsteps

#[macro_use]
extern crate dmoj;

const MOD: usize = 14062008;

fn main() {
    let (n, k) = scan!(usize, usize);

    let mut dp = vec![1; n + 1];
    dp[0] = 0;
    dp[1] = 1;

    let mut l = 0;
    let mut has_solution = true;
    for _ in 0..k {
        let c = scan!(usize);
        dp[c] = 0;
        if l != 0 && l - c == 1 {
            // two consecutive broken floors => no way to reach the last
            has_solution = false;
        }
        l = c;
    }

    if !has_solution {
        println!("0");
        return;
    }

    for i in 2..=n {
        dp[i] = if dp[i] == 0 {
            0
        } else {
            (dp[i - 2] + dp[i - 1]) % MOD
        };
    }

    println!("{}", dp[n]);
}
