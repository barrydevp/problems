// https://oj.vnoi.info/problem/lis

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let n = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let a = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut dp = vec![0; n + 1];

    let mut max = 1;
    dp[1] = a[0];
    for i in 1..n {
        let (mut l, mut r) = (1, max + 1);

        while l < r {
            let m = l + (r - l) / 2;
            if dp[m] >= a[i] {
                r = m;
            } else {
                l = m + 1;
            }
        }

        dp[l] = a[i];

        max = max.max(l);
    }

    println!("{}", max);
}
