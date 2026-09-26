// ==========================================================================
// 典型90 #060  Chimera  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bh
// ==========================================================================
// 【アルゴリズム】 LIS（最長増加部分列）を前から・後ろからの2回。i を頂点とする山型列の長さ = P[i] + Q[i] - 1
// 【計算量】       O(N log N)
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - LIS の定番実装: dp をソート済み配列として持ち、lower_bound の位置を a[i] で上書き
//   - binary_search(&x).unwrap_or_else(|x| x) … 見つかれば Ok(位置)、無ければ Err(挿入位置)
//   - 同名変数 dp の再宣言（シャドーイング）で2回目の DP 用に作り直している
//
// 【Pythonで書くと（考え方の対応）】
//   from bisect import bisect_left
//   def lis_ends(a):
//       dp = []; res = []
//       for x in a:
//           k = bisect_left(dp, x)
//           if k == len(dp): dp.append(x)
//           else: dp[k] = x
//           res.append(k + 1)
//       return res
//   P = lis_ends(A); Q = lis_ends(A[::-1])[::-1]
//   print(max(p + q - 1 for p, q in zip(P, Q)))
//
// 【注意・改善ポイント】
//   ! use BinaryHeap は未使用（warning）。lis_len / lis_len_rev も計算しているが未使用。
//   ! binary_search は同じ値が複数あるとどれを返すか保証しない。ここでは dp の有限値が狭義単調増加なので問題ないが、
//     一般には partition_point(|&v| v < x) の方が安全（= bisect_left）。
// ==========================================================================

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
    // i32::MAX で埋めた配列を使うと、Py の「末尾なら append」を分岐なしで書ける。
    let mut dp = vec![i32::MAX; n + 1];
    let mut p = vec![0; n];
    let mut lis_len = 0;

    for i in 0..n {
        // Ok(p) でも Err(p) でも p を取り出す。lower_bound の位置。
        let pos = dp.binary_search(&a[i]).unwrap_or_else(|x| x);
        dp[pos] = a[i];
        // p[i] = a[i] で終わる増加部分列の最大長。
        p[i] = pos + 1;
        lis_len = cmp::max(lis_len, p[i]);
    }

    // Step 2: Calculate LIS length for decreasing subsequence from the end
    // シャドーイング: 同名の dp を新しく作る。前の dp はここで使えなくなる。
    let mut dp = vec![i32::MAX; n + 1];
    let mut q = vec![0; n];
    let mut lis_len_rev = 0;

    // 後ろから LIS = 前から見ると「i から始まる減少列」の長さ。
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
