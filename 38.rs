use proconio::input;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let threshold: u64 = 1_000_000_000_000_000_000;
    let c = a / gcd(a, b);

    if b <= threshold / c {
        println!("{}", c * b);
    } else {
        println!("Large");
    }
}
