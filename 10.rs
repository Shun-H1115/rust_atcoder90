// ==========================================================================
// 典型90 #010  Score Sum Queries  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_j
// ==========================================================================
// 【アルゴリズム】 累積和（クラス別に prefix sum を作り区間和を O(1) で答える）
// 【計算量】       O(N + Q)
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - タプルの配列入力 [(usize, i32); n]
//   - let (c, p) = cp[i - 1]; … タプルの分解代入（Py と同じ）
//   - 1..=n … 1 から n まで（n を含む）
//   - for &(l, r) in &lr … 参照を借りて走査しつつパターンで分解
//
// 【Pythonで書くと（考え方の対応）】
//   s1 = [0]*(n+1); s2 = [0]*(n+1)
//   for i, (c, p) in enumerate(cp, 1):
//       s1[i] = s1[i-1] + (p if c == 1 else 0)
//       s2[i] = s2[i-1] + (p if c == 2 else 0)
//   for l, r in lr:
//       print(s1[r] - s1[l-1], s2[r] - s2[l-1])
//
// 【注意・改善ポイント】
//   ! Q が最大 1e5 行の出力。Rust の stdout は行バッファなので println! は毎行 flush とロック取得が走り遅い。
//     大量出力は BufWriter + writeln! を使う習慣をつけよう（#004 の注意参照）。
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        cp: [(usize, i32); n],  // Each entry contains C[i] and P[i]
        q: usize,
        lr: [(usize, usize); q],  // Each query contains L[i] and R[i]
    }

    // Step #2: Initialize cumulative sums for class 1 and class 2
    // sum1[0] = 0 を番兵にすると、区間 [l, r] の和が sum1[r] - sum1[l-1] で書ける。
    let mut sum1 = vec![0; n + 1];
    let mut sum2 = vec![0; n + 1];
    
    for i in 1..=n {
        // i32 は Copy なので、タプルの分解で値がコピーされる。
        let (c, p) = cp[i - 1];
        sum1[i] = sum1[i - 1];
        sum2[i] = sum2[i - 1];
        if c == 1 {
            sum1[i] += p;
        } else if c == 2 {
            sum2[i] += p;
        }
    }

    // Step #3: Process each query and output the result
    // &lr で借用して走査、&(l, r) で参照を外して分解。
    for &(l, r) in &lr {
        let answer1 = sum1[r] - sum1[l - 1];
        let answer2 = sum2[r] - sum2[l - 1];
        println!("{} {}", answer1, answer2);
    }
}
