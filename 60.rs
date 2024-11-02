use proconio::input;
use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    // Input reading
    input! {
        n: usize,
        a: [i32; n],
    }

    // Step 1: Calculate LIS length for increasing subsequence
    let mut dp = vec![i32::MAX; n + 1];
    let mut p = vec![0; n];
    let mut lis_len = 0;

    for i in 0..n {
        let pos = dp.binary_search(&a[i]).unwrap_or_else(|x| x);
        dp[pos] = a[i];
        p[i] = pos + 1;
        lis_len = cmp::max(lis_len, p[i]);
    }

    // Step 2: Calculate LIS length for decreasing subsequence from the end
    let mut dp = vec![i32::MAX; n + 1];
    let mut q = vec![0; n];
    let mut lis_len_rev = 0;

    for i in (0..n).rev() {
        let pos = dp.binary_search(&a[i]).unwrap_or_else(|x| x);
        dp[pos] = a[i];
        q[i] = pos + 1;
        lis_len_rev = cmp::max(lis_len_rev, q[i]);
    }

    // Step 3: Find the longest bitonic subsequence
    let mut answer = 0;
    for i in 0..n {
        answer = cmp::max(answer, p[i] + q[i] - 1);
    }
    println!("{}", answer);
}
