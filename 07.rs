// ==========================================================================
// 典型90 #007  CP Classes  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_g
// ==========================================================================
// 【アルゴリズム】 ソート＋二分探索（lower_bound の位置とその1つ前を比較）
// 【計算量】       O((N+Q) log N)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - input! の中で mut a: [i32; n] と書くと可変で受け取れる（sort するため）
//   - binary_search_by + then(Ordering::Greater) … Rust で lower_bound を書くイディオム
//   - unwrap_or_else(|x| x) … Result の Ok/Err どちらでも中身を取り出す
//   - （Rust 1.52+ なら a.partition_point(|&x| x < b_i) の方が読みやすい。Py の bisect_left と同じ意味）
//
// 【Pythonで書くと（考え方の対応）】
//   from bisect import bisect_left
//   a.sort()
//   for b in B:
//       pos = bisect_left(a, b)
//       cand = []
//       if pos < n: cand.append(abs(b - a[pos]))
//       if pos > 0: cand.append(abs(b - a[pos-1]))
//       print(min(cand))
// ==========================================================================

use proconio::input;
// 複数まとめて use できる。min は std::cmp::min（Py の組み込み min 相当、ただし2引数）。
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
    // a.sort() … Py の list.sort() と同じく破壊的。mut が必要な理由。
    a.sort();

    // Step #3. Binary Search
    for &b_i in &b {
        // 【lower_bound イディオム】比較で Equal を返さず Greater 扱いにすると、binary_search は絶対に
        // Ok を返さず、Err(挿入位置＝最初の b_i 以上の位置) を返す。= Py の bisect_left。
        // 推奨: let pos = a.partition_point(|&x| x < b_i);
        let pos = a.binary_search_by(|&x| x.cmp(&b_i).then(Ordering::Greater)).unwrap_or_else(|x| x);

        // if は式なので値を返せる。Py: diff1 = abs(b - a[pos]) if pos < n else INF
        let diff1 = if pos < n { (b_i - a[pos]).abs() } else { INF };
        // pos > 0 を先に確認してから pos - 1。usize の 0 - 1 は panic するので必ずガードする。
        let diff2 = if pos > 0 { (b_i - a[pos - 1]).abs() } else { INF };

        // min(diff1, diff2) … std::cmp::min。f64 には使えない（f64 は a.min(b) を使う）。
        println!("{}", min(diff1, diff2));
    }
}
