use proconio::input;

fn solve(m: i64, n: usize, k: i64, l: i64, a: &[i64]) -> bool {
    let mut cnt = 0;
    let mut pre = 0;

    for i in 0..n {
        if a[i] - pre >= m && l - a[i] >= m {
            cnt += 1;
            pre = a[i];
        }
    }

    cnt >= k
}

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        l: i64,
        k: i64,
        a: [i64; n],
    }

    // Step #2. Binary search
    let mut left = 0;
    let mut right = l + 1;

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if solve(mid, n, k, l, &a) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
