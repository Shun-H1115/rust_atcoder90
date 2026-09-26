// ==========================================================================
// 典型90 #022  Cubic Cake  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_v
// ==========================================================================
// 【アルゴリズム】 最大公約数（3辺の gcd の立方体に切るのが最適）
// 【計算量】       O(log max(A,B,C))
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - fn gcd(mut a: i64, mut b: i64) … 引数に mut を付けると関数内で書き換え可能
//   - while ループ
//   - 標準ライブラリに gcd は無い（Py の math.gcd）ので自作する。ライブラリ化推奨
//
// 【Pythonで書くと（考え方の対応）】
//   from math import gcd
//   s = gcd(gcd(a, b), c)
//   print(a//s - 1 + b//s - 1 + c//s - 1)
// ==========================================================================

use proconio::input;

// 引数の mut … 引数は既定で不変なので、ループ内で書き換えるために mut を付ける。
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        // Py なら a, b = b, a % b の1行。Rust でも (a, b) = (b, a % b); と書ける（Rust 1.59+ の分割代入）。
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let s = gcd(a, gcd(b, c));
    // 各辺を s ごとに切る回数は (辺/s - 1)。
    let result = (a / s - 1) + (b / s - 1) + (c / s - 1);
    println!("{}", result);
}
