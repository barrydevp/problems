// https://oj.vnoi.info/problem/atcoder_dp_k

#[macro_use]
extern crate dmoj;

fn main() {
    let (n, k) = scan!(usize, usize);

    let a = (0..n).map(|_| scan!(usize)).collect::<Vec<usize>>();

    // dp[i] is: if a person starting with i stones, can he win?
    // dp[i] = true (this person starting with i stones wins) if at least one case the next
    //         person starting with i-a_j stones loses (the a_j that make the next person lose
    //         will be the next action of the start person) (Game theory?)
    let mut dp = vec![0; k + 1];

    for i in 0..=k {
        for &aj in &a {
            if i >= aj {
                dp[i] |= dp[i - aj] ^ 1;
            }
        }
    }

    println!("{}", if dp[k] == 1 { "First" } else { "Second" });
}
