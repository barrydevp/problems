#[macro_use]
extern crate dmoj;

const N: usize = 33;
const M: usize = 33;

fn main() {
    // create dp table
    let mut dp = [[0 as usize; N]; M];

    // base cases
    for i in 1..N {
        dp[i][1] = 1;
        dp[1][i] = i;
    }

    for i in 2..N {
        for j in 2..M {
            dp[i][j] = 1 + dp[i][j - 1] + dp[i - 1][j - 1];
        }
    }

    // read test
    let nt = scan!(usize);

    for _ in 0..nt {
        let (m, n) = scan!(usize, usize);
        if m > dp[n][32] {
            println!("Impossible");
            continue;
        }

        // let mut found = false;
        // for i in 1..=32 {
        //     if dp[n][i] >= m {
        //         println!("{}", i);
        //         found = true;
        //         break;
        //     }
        // }

        // if !found {
        //     println!("Impossible");
        // }

        // since dp[n][i] is increasing, we can use binary search to find the answer
        let mut l = 1;
        let mut r = 32;
        while l < r {
            let mid = (l + r) / 2;

            if dp[n][mid] >= m {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        println!("{}", l.min(r));
    }
}
