fn main() {
    // read input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // parse input to int
    let nt = input.trim().parse::<usize>().unwrap();

    for _ in 0..nt {
        let mut input = String::new();
        // read input from stdin
        std::io::stdin().read_line(&mut input).unwrap();
        // split input by space and parse to int, the input is a string in form of "a b"
        let mut iter = input.trim().split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let m = iter.next().unwrap().parse::<usize>().unwrap();

        println!("{} {}", n, m);
    }
}
