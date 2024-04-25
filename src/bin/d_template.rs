#[macro_use]
extern crate dmoj;

fn main() {
    // number of test
    let nt = scan!(usize);

    // read test
    for _ in 0..nt {
        let (m, n) = scan!(usize, usize);
        println!("{} {}", m, n);
    }
}
