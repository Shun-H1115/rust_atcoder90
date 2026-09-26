// ==========================================================================
// 典型90 #016  Minimum Coins  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_p
// ==========================================================================
// 【アルゴリズム】 全探索（A硬貨とB硬貨の枚数を二重ループ、C硬貨の枚数は割り算で決まる）
// 【計算量】       O(9999^2 / 2)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - i64::MAX … 型の最大値定数（Py の float('inf') 代わり）
//   - 0..=9999 - i … 範囲の上限に式を書ける
//   - continue / 早期スキップ
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 10**18
//   for i in range(10000):
//       for j in range(10000 - i):
//           v = N - i*A - j*B
//           if v < 0 or v % C: continue
//           ans = min(ans, i + j + v // C)
//   print(ans)
// ==========================================================================

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    // i64::MAX を初期値にしている。ここでは足し算しないので溢れの心配なし。
    let mut answer = i64::MAX;

    // i, j は n と演算するので i64 に推論される。約5000万回のループだが Rust なら余裕（Py だと TLE）。
    for i in 0..=9999 {
        for j in 0..=9999 - i {
            // v < 0 のチェックを先にするのは i64 だから可能。usize だとここで panic する。
            let v = n - i * a - j * b;
            if v < 0 || v % c != 0 {
                continue;
            }
            // Rust の整数除算 / は 0 方向への切り捨て（Py の // は負の無限大方向）。v >= 0 なのでここでは同じ。
            let r = i + j + v / c;
            if r <= 9999 {
                answer = min(answer, r);
            }
        }
    }

    println!("{}", answer);
}
