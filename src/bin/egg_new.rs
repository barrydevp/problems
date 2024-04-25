const N: usize = 1001;
const M: usize = 1001;

fn main() {
    // create dp table
    let mut dp = vec![vec![0; M]; N];

    // base cases
    for i in 0..N-1 {
        dp[0][i] = 1;
        dp[i][0] = 1;
    }
    // for i in 1..M {
    //     dp[1][i] = i;
    // }

    for i in 1..N-1 {
        for j in 1..M-1 {
            dp[i][j] = dp[i][j - 1] + dp[i - 1][j - 1];
            // if dp[i][j] >= M {
            //     break;
            // }
        }
    }

    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let nt = input.trim().parse::<usize>().unwrap();

    for _ in 0..nt {
        // read input from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // split input by space and parse to int, the input is a string in form of "a b"
        let mut iter = input.trim().split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let m = iter.next().unwrap().parse::<usize>().unwrap();

        // for i in 1..=m {
        //     if dp[n][i] >= m {
        //         println!("{}", i);
        //         break;
        //     }
        // }

        // since dp[n][i] is increasing, we can use binary search to find the answer
        let mut l = 1;
        let mut r = m;
        while l < r {
            let mid = (l + r) / 2;
            // println!("l: {}, r: {}, mid: {}, dp: {}", l, r, mid, dp[n][mid]);
            // if dp[n][mid] >= m && dp[n][mid - 1] <= m {
            //     println!("{}", mid);
            //     break;
            // }

            if dp[n][mid] >= m || dp[n][mid] == 0 {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        println!("{}", l.min(r));
    }
}
