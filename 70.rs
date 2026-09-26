// ==========================================================================
// 典型90 #070  Plant Planning  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_br
// ==========================================================================
// 【アルゴリズム】 マンハッタン距離の総和は x と y で独立 → それぞれ中央値が最適
// 【計算量】       O(N)（select_nth_unstable は平均線形）
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - select_nth_unstable(k) … k 番目の要素を正しい位置に置く（全体ソートより速い。Py: statistics.median_low 等）
//   - i128 … 128bit 整数（Py の多倍長の代わりに、溢れが心配なときの保険）
//   - pts.iter().map(|&(x, _)| x).collect() … タプルの片方だけ取り出す（Py: [x for x, _ in pts]）
//
// 【Pythonで書くと（考え方の対応）】
//   xs = sorted(x for x, _ in P); ys = sorted(y for _, y in P)
//   mx, my = xs[n//2], ys[n//2]
//   print(sum(abs(x - mx) + abs(y - my) for x, y in P))
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        pts: [(i64, i64); n],
    }

    // |&(x, _)| … タプルを分解して x だけ使う。
    let mut xs: Vec<i64> = pts.iter().map(|&(x, _)| x).collect();
    let mut ys: Vec<i64> = pts.iter().map(|&(_, y)| y).collect();

    let mid = n / 2;
    // select_nth_unstable は mid 番目だけを確定させる（左は小さい、右は大きい）。ソートより軽い。
    xs.select_nth_unstable(mid);
    ys.select_nth_unstable(mid);
    let mx = xs[mid];
    let my = ys[mid];

    // i128 にしているが、|x| <= 1e9 × N = 1e5 程度なら総和 2e14 程度で i64 でも十分。
    let mut ans: i128 = 0;
    for &(x, y) in &pts {
        ans += (x - mx).abs() as i128;
        ans += (y - my).abs() as i128;
    }

    println!("{}", ans);
}
