// https://oj.vnoi.info/problem/nkcable

#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);

    let a = (0..n - 1).map(|_| scan!(usize)).collect::<Vec<usize>>();

    // the dp[i] is: what is the minimum wire length needed for connect i computers (computer i always connected with i-1)
    // so when deciding value for dp[i], we need to answer:
    // - Should we connect computer (i-2, i-1) + (i-1, i) or (i-3, i-2) + (i-1, i) 
    // - Since with dp[i] there will be a connection (i-1, i), so dp[i-1] always has connection (i-2, i-1)
    //   and dp[i-2] always has connection (i-2, i-3)
    // You may wonder obviously dp[i-2] < dp[i-1] but no, don't imagine that at dp[i-1] there will be same connection at dp[i-2]
    // for example, at dp[i-1] there may not have connection (i-2, i-3) but at dp[i-2] there always has (i-2, i-3) and that make
    // the total length different
    let mut dp = vec![0; n + 1];
    // why we set 1 computer here? because there will be at least 2 computers so dp[1] will never happen and we always have connection
    // between computer 1 and 2. We set this to make sure that at futher decisions, computer 1 and 2 is always connected.
    // Of course we could set dp[1] to any value >= dp[2].
    dp[1] = a[0];
    dp[2] = a[0];

    for i in 3..=n {
        dp[i] = (dp[i - 1]).min(dp[i - 2]) + a[i - 2];
    }

    println!("{}", dp[n]);
}
