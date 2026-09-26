// ==========================================================================
// 典型90 #048  I will not drop out  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_av
// ==========================================================================
// 【アルゴリズム】 貪欲法（部分点 B と 残り A-B を独立な得点として並べ、大きい順に K 個取る。B >= A-B なので順序は自動的に守られる）
// 【計算量】       O(N log N)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - sort_unstable_by(|a, b| b.cmp(a)) … 降順ソート。Py: sort(reverse=True)
//   - （別解）vec.sort_unstable(); vec.reverse(); や sort_by_key(|&x| Reverse(x))
//   - iter().take(k).sum() … Py: sum(v[:k])
//   - sort_unstable は安定性を保証しない代わりに速い（数値なら違いは出ない）
//
// 【Pythonで書くと（考え方の対応）】
//   v = []
//   for a, b in AB: v += [b, a - b]
//   v.sort(reverse=True)
//   print(sum(v[:K]))
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(i64, i64); n],
    }
    
    // Step #1. Populate the `vec` with values `B[i]` and `A[i] - B[i]`
    // Vec::new() … 要素型は push した b（i64）から推論。変数名 vec はマクロ vec! と別物なので使えるが紛らわしい。
    let mut vec = Vec::new();
    for (a, b) in ab {
        vec.push(b);
        vec.push(a - b);
    }
    
    // Step #2. Sort `vec` in descending order and sum the top `K` elements
    // b.cmp(a) と引数を逆にすると降順。
    vec.sort_unstable_by(|a, b| b.cmp(a));  // Sorting in descending order
    // take(k) … 先頭 k 個だけ取るイテレータ。sum は型注釈 : i64 で結果の型を指定。
    let answer: i64 = vec.iter().take(k).sum();

    // Step #3. Output the answer
    println!("{}", answer);
}
