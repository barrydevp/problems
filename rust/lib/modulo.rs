const MOD: i64 = 1000000007;

// Extended Euclidean Algorithm
// return (d, x, y) where d = gcd(a, b) = a * x + b * y
fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x1, y1) = gcd(b, a % b);
        (d, y1, x1 - y1 * (a / b))
    }
}

// Fast bound using binary
fn pow(base: i64, power: i64) -> i64 {
    let mut base = base;
    let mut power = power;
    let mut r = 1;
    while power > 0 {
        if power % 2 == 0 {
            base = base * base;
        } else {
            power -= 1;
            r = (r * base) % 2
        }
        power /= 2;
    }

    r
}
