// https://oj.vnoi.info/problem/bedao_g16_a

#[macro_use]
extern crate dmoj;
use std::collections::HashMap;

fn main() {
    let dis = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -2),
        (0, -1),
        (0, 0),
        (0, 1),
        (0, 2),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, 0),
    ];
    let n = scan!(usize);

    let mut a = HashMap::<(i32, i32), u32>::with_capacity(n);
    for _ in 0..n {
        let p = (scan!(i32), scan!(i32));
        *a.entry(p).or_insert(0) += scan!(u32);
    }

    // println!("{:?}", a);
    let mut max = 0;
    a.keys().for_each(|p| {
        for d1 in dis {
            let c = (p.0 - d1.0, p.1 - d1.1);
            let mut val: u64 = 0;
            for d2 in dis {
                if let Some(&v) = a.get(&(c.0 + d2.0, c.1 + d2.1)) {
                    val += v as u64;
                }
            }
            max = max.max(val);
        }
    });

    println!("{}", max);
}
