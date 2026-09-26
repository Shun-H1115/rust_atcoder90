// ==========================================================================
// 典型90 #018  Statue of Chokudai  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_r
// ==========================================================================
// 【アルゴリズム】 幾何（観覧車の位置を三角関数で求め、atan2 で俯角を計算）
// 【計算量】       O(Q)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - f64 のメソッド群: x.sin(), x.cos(), x.powi(2), x.sqrt(), y.atan2(x) … Py の math.sin(x) 等は関数、Rust はメソッド
//   - std::f64::consts::PI … Py: math.pi
//   - 浮動小数点リテラルは 2.0 のように . を付ける（2 だと整数で型エラー）
//
// 【Pythonで書くと（考え方の対応）】
//   import math
//   def query(e):
//       th = e / T * 2 * math.pi
//       cy = -(L/2) * math.sin(th); cz = L/2 - (L/2) * math.cos(th)
//       d1 = math.hypot(X - 0, Y - cy)
//       return math.degrees(math.atan2(cz, d1))
// ==========================================================================

use proconio::input;
use std::f64::consts::PI;

fn query(t: f64, l: f64, x: f64, y: f64, e: f64) -> f64 {
    let cx = 0.0;
    // (e / t * 2.0 * PI).sin() … 式全体を括弧でくくってメソッド呼び出し。
    let cy = -(l / 2.0) * (e / t * 2.0 * PI).sin();
    let cz = (l / 2.0) - (l / 2.0) * (e / t * 2.0 * PI).cos();
    // powi(2) は整数乗（高速）。実数乗は powf。代替: (dx).hypot(dy) で sqrt(dx^2+dy^2)。
    let d1 = ((cx - x).powi(2) + (cy - y).powi(2)).sqrt();
    let d2 = cz;
    // d2.atan2(d1) は Py の math.atan2(d2, d1) と同じ意味（レシーバが y 側）。引数順に注意！
    let angle = d2.atan2(d1);
    // 度に変換。angle.to_degrees() とも書ける。
    angle * 180.0 / PI
}

fn main() {
    // Step #1: Input
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e_values: [f64; q],
    }

    // Step #2: Process each query and output the result
    // for &e in &e_values … f64 は Copy なので &e で値として受け取れる。
    for &e in &e_values {
        println!("{:.12}", query(t, l, x, y, e));
    }
}
