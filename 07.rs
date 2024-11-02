use proconio::input;
use std::cmp::{min, Ordering};

const INF: i32 = 2_000_000_000;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        b: [i32; q],
    }

    // Step #2. Sorting
    a.sort();

    // Step #3. Binary Search
    for &b_i in &b {
        let pos = a.binary_search_by(|&x| x.cmp(&b_i).then(Ordering::Greater)).unwrap_or_else(|x| x);

        let diff1 = if pos < n { (b_i - a[pos]).abs() } else { INF };
        let diff2 = if pos > 0 { (b_i - a[pos - 1]).abs() } else { INF };

        println!("{}", min(diff1, diff2));
    }
}