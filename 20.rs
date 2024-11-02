use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let mut e = 1u64;
    for _ in 0..b {
        if e > a / c {
            // If `e * c` would exceed `a`, we know `A < C^B` will be true
            println!("Yes");
            return;
        }
        e *= c;
    }

    if a < e {
        println!("Yes");
    } else {
        println!("No");
    }
}
