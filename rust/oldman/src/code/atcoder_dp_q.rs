// https://oj.vnoi.info/problem/atcoder_dp_q

#[macro_use]
extern crate dmoj;

fn update(bit: &mut Vec<u64>, i: usize, val: u64) {
    let mut i = i;
    while i < bit.len() {
        bit[i] = bit[i].max(val);
        let _i = i as i32;
        i += (_i & (-_i)) as usize;
    }
}

fn get(bit: &[u64], i: usize) -> u64 {
    let mut i = i;
    let mut r = 0;
    while i > 0 {
        r = r.max(bit[i]);
        let _i = i as i32;
        i -= (_i & (-_i)) as usize;
    }

    r
}

fn main() {
    let n = scan!(usize);

    let h = (0..n).map(|_| scan!(usize)).collect::<Vec<usize>>();
    let a = (0..n).map(|_| scan!(usize)).collect::<Vec<usize>>();

    let mut bit = vec![0_u64; n + 1];

    for i in 0..n {
        let v = get(&bit, h[i] - 1);
        update(&mut bit, h[i], v + a[i] as u64);
    }

    println!("{}", get(&bit, n));
}
