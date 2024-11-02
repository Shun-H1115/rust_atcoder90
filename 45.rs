use proconio::input;
use std::cmp::{max, min};
use std::i64::MAX;

fn main() {
    input! {
        n: usize,
        k: usize,
        points: [(i32, i32); n],
    }

    // Calculate pairwise distances squared between points
    let mut d = vec![vec![0_i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            let dx = (points[i].0 - points[j].0) as i64;
            let dy = (points[i].1 - points[j].1) as i64;
            d[i][j] = dx * dx + dy * dy;
        }
    }

    // Calculate cost for each subset of points
    let mut cost = vec![0_i64; 1 << n];
    for i in 1..(1 << n) {
        for j in 0..n {
            for k in 0..j {
                if ((i >> j) & 1) == 1 && ((i >> k) & 1) == 1 {
                    cost[i] = max(cost[i], d[j][k]);
                }
            }
        }
    }

    // Initialize dp array with a high initial value
    let mut dp = vec![vec![MAX; 1 << n]; k + 1];
    dp[0][0] = 0;

    // Dynamic programming to minimize the maximum distance
    for i in 1..=k {
        for j in 1..(1 << n) {
            let mut subset = j;
            while subset != 0 {
                dp[i][j] = min(dp[i][j], max(dp[i - 1][j - subset], cost[subset]));
                subset = (subset - 1) & j;
            }
        }
    }

    // Output the minimum possible value for k groups covering all points
    println!("{}", dp[k][(1 << n) - 1]);
}
