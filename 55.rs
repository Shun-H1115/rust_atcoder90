use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        a: [i64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        if a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
