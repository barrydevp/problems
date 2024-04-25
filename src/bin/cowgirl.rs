// const V: [usize; 2] = [0, 15];

fn check_fn(a: usize, b: usize, n: usize) -> bool {
    // for i in 0..(n - 1) {
    //     if V.contains(&(((a >> i) & 3) | (((b >> i) & 3) << 2))) {
    //         return false;
    //     }
    // }
    //
    // return true;

    let x = a & b;
    let y = (!a & !b) & ((1 << n) - 1);

    (x & x >> 1) == 0 && (y & y >> 1) == 0
}

fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let tc = input.trim().parse::<usize>().unwrap();
    for _ in 0..tc {
        // read input from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // split input by space and parse to int, the input is a string in form of "a b"
        let mut iter = input.trim().split_whitespace();
        let mut m = iter.next().unwrap().parse::<usize>().unwrap();
        let mut n = iter.next().unwrap().parse::<usize>().unwrap();
        // swap m and n if m > n
        if m < n {
            (m, n) = (n, m);
        }

        let mut f = vec![vec![0; 1 << n as usize]; m as usize];
        let mut check = vec![vec![true; 1 << n as usize]; 1 << n as usize];
        for i in 0..(1 << n) {
            for j in (i)..(1 << n) {
                check[i][j] = check_fn(i, j, n);
            }
        }

        let mut sum = 0;

        for i in 0..m {
            for j in 0..(1 << n) {
                if i == 0 {
                    f[i][j] = 1;
                } else {
                    // compute f[i][j] = sum from 0 to (1<<n) of f[i-1][k] if check[j][k] is true
                    for k in 0..(1 << n) {
                        if check[j][k] && check[k][j] {
                            f[i][j] += f[i - 1][k];
                        }
                    }
                }
                if i == (m - 1) {
                    sum += f[i][j];
                }
            }
        }

        println!("{}", sum);
    }
}
