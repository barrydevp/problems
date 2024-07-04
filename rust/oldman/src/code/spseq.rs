// https://oj.vnoi.info/problem/spseq

#[macro_use]
extern crate dmoj;

fn lxq<F>(a: &[usize], cmp: F) -> Vec<usize>
where
    F: Fn(usize, usize) -> bool,
{
    let mut v = vec![0; a.len() + 1];
    let mut o = vec![0; a.len() + 1];
    o[1] = a[0];
    v[1] = 1;

    for i in 2..=a.len() {
        let mut l = 1;
        let mut r = v[i - 1] + 1;

        while l < r {
            let m = l + (r - l) / 2;
            if cmp(a[i - 1], o[m]) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        o[l] = a[i - 1];
        v[i] = v[i - 1].max(l);
    }

    v
}

fn main() {
    let n = scan!(usize);
    let mut a = (0..n).map(|_| scan!(usize)).collect::<Vec<usize>>();

    let lis = lxq(&a, |a1, a2| a1 > a2);
    a.reverse();
    // println!("{:?}", a);
    let lds = lxq(&a, |a1, a2| a1 > a2);

    let mut r = 0;
    for i in 1..n {
        r = r.max(lis[i].min(lds[n-i+1]) * 2 - 1);
    }
    // println!("{:?}", lis);
    // println!("{:?}", lds);
    // println!("{:?}", dp);

    println!("{}", r);
}
