use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut answer = i64::MAX;

    for i in 0..=9999 {
        for j in 0..=9999 - i {
            let v = n - i * a - j * b;
            if v < 0 || v % c != 0 {
                continue;
            }
            let r = i + j + v / c;
            if r <= 9999 {
                answer = min(answer, r);
            }
        }
    }

    println!("{}", answer);
}
