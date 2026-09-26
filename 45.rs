// ==========================================================================
// 典型90 #045  Simple Grouping  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_as
// ==========================================================================
// 【アルゴリズム】 bitDP＋部分集合列挙（dp[k][S] = 集合 S を k グループに分けたときの最大距離の最小値）
// 【計算量】       O(K × 3^N)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - 部分集合列挙: subset = (subset - 1) & j … Py と同じビットトリック
//   - points[i].0 … タプルの要素アクセス（Py の points[i][0]）
//   - use std::i64::MAX; は古い書き方。今は i64::MAX を直接使う
//   - vec![0_i64; 1 << n] … 2^n 要素
//
// 【Pythonで書くと（考え方の対応）】
//   cost = [0]*(1<<n)
//   for S in range(1, 1<<n):
//       cost[S] = max((d[j][k] for j in range(n) for k in range(j) if S>>j&1 and S>>k&1), default=0)
//   dp = [[INF]*(1<<n) for _ in range(K+1)]; dp[0][0] = 0
//   for i in range(1, K+1):
//       for S in range(1, 1<<n):
//           T = S
//           while T:
//               dp[i][S] = min(dp[i][S], max(dp[i-1][S^T], cost[T])); T = (T-1) & S
// ==========================================================================

use proconio::input;
use std::cmp::{max, min};
// （非推奨）std::i64::MAX は旧式。i64::MAX と書けば use 不要。
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
            // i32 同士で引き算してから i64 に変換。座標が大きいと二乗で溢れるので i64 で掛ける。
            let dx = (points[i].0 - points[j].0) as i64;
            let dy = (points[i].1 - points[j].1) as i64;
            d[i][j] = dx * dx + dy * dy;
        }
    }

    // Calculate cost for each subset of points
    let mut cost = vec![0_i64; 1 << n];
    for i in 1..(1 << n) {
        for j in 0..n {
            // 変数名 k が input の k を一時的に隠している（シャドーイング）。ループ内の k は別物なので注意。
            for k in 0..j {
                if ((i >> j) & 1) == 1 && ((i >> k) & 1) == 1 {
                    cost[i] = max(cost[i], d[j][k]);
                }
            }
        }
    }

    // Initialize dp array with a high initial value
    // i64::MAX を INF に使用。max(...) を取るだけで足し算しないので溢れない。
    let mut dp = vec![vec![MAX; 1 << n]; k + 1];
    dp[0][0] = 0;

    // Dynamic programming to minimize the maximum distance
    for i in 1..=k {
        for j in 1..(1 << n) {
            let mut subset = j;
            while subset != 0 {
                // j - subset は j ^ subset と同じ（subset ⊆ j なので）。
                dp[i][j] = min(dp[i][j], max(dp[i - 1][j - subset], cost[subset]));
                // 次に小さい部分集合へ。0 になったら終了。
                subset = (subset - 1) & j;
            }
        }
    }

    // Output the minimum possible value for k groups covering all points
    println!("{}", dp[k][(1 << n) - 1]);
}
