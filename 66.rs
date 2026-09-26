// ==========================================================================
// 典型90 #066  Various Arrays  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bn
// ==========================================================================
// 【アルゴリズム】 期待値の線形性（転倒数の期待値 = 各ペア (i<j) で A_i > A_j となる確率の総和）
// 【計算量】       O(N^2 × 値の範囲)
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - f64 の累積: let mut expsum = 0.0;（リテラルに . を付けると f64）
//   - (ri - li + 1) as f64 … 整数→浮動小数点の明示変換（Py は自動）
//   - li.max(lj) … メソッド版 max
//   - {:.15} … 小数15桁出力
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 0.0
//   for i in range(n):
//       for j in range(i+1, n):
//           (li, ri), (lj, rj) = R[i], R[j]
//           cnt = sum(min(max(k - lj, 0), rj - lj + 1) for k in range(li, ri+1))
//           ans += cnt / ((ri-li+1) * (rj-lj+1))
//   print(f'{ans:.15f}')
//
// 【注意・改善ポイント】
//   ! overlap_min / overlap_max は未使用、count = 0.0 の再代入も不要（warning）。
//     let count = if ... { 0.0 } else if ... { 1.0 } else { ... }; と if 式で書くと mut も不要になる。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,          // Number of intervals
        ranges: [(i32, i32); n],  // Vector of (L, R) pairs representing ranges
    }

    // 0.0 なので f64。0 と書くと整数になり、後で f64 を足すところで型エラー。
    let mut expsum = 0.0;

    // Loop over each pair of intervals
    for i in 0..n {
        for j in (i + 1)..n {
            let (li, ri) = ranges[i];  // Range [L[i], R[i]]
            let (lj, rj) = ranges[j];  // Range [L[j], R[j]]
            // Rust は整数と f64 を混ぜて計算できない。as f64 で明示変換。
            let len_i = (ri - li + 1) as f64;
            let len_j = (rj - lj + 1) as f64;

            let mut count = 0.0;

            // Calculate the probability of values in [L[i], R[i]] being greater than in [L[j], R[j]]
            if ri < lj {
                // All values in range i are less than all in range j
                count = 0.0;
            } else if rj < li {
                // All values in range j are less than all in range i
                count = 1.0;
            } else {
                // Handle overlapping ranges
                let overlap_min = li.max(lj);
                let overlap_max = ri.min(rj);

                // Sum probabilities where a number from range i is greater than a number from range j
                // A_i = k のとき、A_j < k となる A_j の個数を数える。
                for k in li..=ri {
                    if k > rj {
                        // If k in range i is greater than all values in range j
                        count += len_j;
                    } else if k >= lj {
                        // If k in range i is within the overlap with range j
                        // (k - lj) as f64 … [lj, k) の個数。
                        count += (k - lj) as f64;
                    }
                }
                // 場合の数 ÷ 全組合せ数 = 確率。
                count /= len_i * len_j;
            }

            // Add the probability for this pair to the total expected sum
            expsum += count;
        }
    }

    // Print the result with 15 decimal precision
    println!("{:.15}", expsum);
}
