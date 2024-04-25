fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let nt = input.trim().parse::<usize>().unwrap();
    let mut tc = vec![[0, 0]; nt];

    // get max egg and max floor
    let mut max_e = 0;
    let mut max_f = 0;
    for i in 0..nt {
        // read input from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // split input by space and parse to int, the input is a string in form of "a b"
        let mut iter = input.trim().split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        max_e = max_e.max(n);
        let m = iter.next().unwrap().parse::<usize>().unwrap();
        max_f = max_f.max(m);
        tc[i][0] = n;
        tc[i][1] = m;
    }

    // create dp table
    let mut dp = vec![vec![0; max_f + 1]; max_e + 1];

    // base cases
    for i in 1..=max_e {
        dp[i][1] = 1;
    }
    for i in 1..=max_f {
        dp[1][i] = i;
    }

    // fill dp table
    for i in 2..=max_e {
        for j in 2..=max_f {
            dp[i][j] = 1 + dp[i][j - 1] + dp[i - 1][j - 1];
        }
    }

    // print result for each test case
    for t in tc {
        for i in 1..=t[1] {
            if dp[t[0]][i] >= t[1] {
                println!("{}", i);
                break;
            }
        }
    }
}
