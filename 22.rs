use proconio::input;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let s = gcd(a, gcd(b, c));
    let result = (a / s - 1) + (b / s - 1) + (c / s - 1);
    println!("{}", result);
}
