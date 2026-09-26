// ==========================================================================
// 典型90 #064  Uplift  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bl
// ==========================================================================
// 【アルゴリズム】 階差（隣接差分 B[i] = A[i+1] - A[i] を管理。区間加算は差分の両端2か所の変化だけ）
// 【計算量】       O(N + Q)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - if 式を2つ + でつなぐ: if l > 0 { .. } else { 0 } + if r < n-1 { .. } else { 0 }
//   - let l = l - 1; … シャドーイングで 0-indexed に変換
//   - usize の境界チェック（l > 0 を確かめてから l - 1）
//
// 【Pythonで書くと（考え方の対応）】
//   b = [a[i+1] - a[i] for i in range(n-1)]
//   tot = sum(map(abs, b))
//   for l, r, v in Q:
//       l -= 1; r -= 1
//       before = (abs(b[l-1]) if l > 0 else 0) + (abs(b[r]) if r < n-1 else 0)
//       if l > 0: b[l-1] += v
//       if r < n-1: b[r] -= v
//       after = ...; tot += after - before; print(tot)
//
// 【注意・改善ポイント】
//   ! Q 行の出力。BufWriter を使うと速い（#059 参照）。
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1: Read input
    input! {
        n: usize,
        q: usize,
        // mut a と受けているが、a 自体は書き換えていない（mut は不要で warning になる）。
        mut a: [i64; n], // initial heights
        queries: [(usize, usize, i64); q] // list of queries (L, R, V)
    }

    // Step #2: Initialize the initial inconvenience
    // 差分配列。要素型は a[i+1] - a[i]（i64）から推論。
    let mut b = vec![0; n - 1]; // differences between consecutive areas
    let mut inconvenience = 0;

    for i in 0..n - 1 {
        b[i] = a[i + 1] - a[i];
        inconvenience += b[i].abs();
    }

    // Step #3: Process each query and output the updated inconvenience
    for (l, r, v) in queries {
        // 同名 l でシャドーイング。元の 1-indexed の l は以降見えなくなる。
        let l = l - 1; // convert to 0-based index
        let r = r - 1; // convert to 0-based index

        // Calculate the effect before applying the change
        // 区間 [l, r] に v を足すと、差分は b[l-1] が +v、b[r] が -v だけ変わる。
        let before_change = if l > 0 { b[l - 1].abs() } else { 0 } + if r < n - 1 { b[r].abs() } else { 0 };

        // Update the differences array
        if l > 0 {
            b[l - 1] += v;
        }
        if r < n - 1 {
            b[r] -= v;
        }

        // Calculate the effect after applying the change
        let after_change = if l > 0 { b[l - 1].abs() } else { 0 } + if r < n - 1 { b[r].abs() } else { 0 };

        // Update the total inconvenience
        inconvenience += after_change - before_change;

        // Print the result for the current query
        println!("{}", inconvenience);
    }
}
