fn gcd(mut x: i64, mut y: i64) -> (i64, i64, i64) {
    let (mut u0, mut v0) = (1, 0);
    let (mut u1, mut v1) = (0, 1);
    while y > 0 {
        let q = x / y;
        let mut tmp;
        tmp = u1;
        u1 = u0 - q * u1;
        u0 = tmp;
        tmp = v1;
        v1 = v0 - q * v1;
        v0 = tmp;
        tmp = y;
        y = x % y;
        x = tmp;
    }
    (x, u0, v0)
}

fn main() {
    assert_eq!((1, 3, -2), gcd(5, 7));
    assert_eq!((18, -9, 40), gcd(2 * 3 * 7 * 9 * 11, 6 * 12 * 13));
    assert_eq!((6, -1351389, 189739), gcd(32423940, 230934894));
    assert_eq!((50, 1, -1), gcd(150, 100));
    assert_eq!((1, -49, 74), gcd(151, 100));
}
