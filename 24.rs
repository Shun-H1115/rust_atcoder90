// ==========================================================================
// 典型90 #024  Select +/- One  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_x
// ==========================================================================
// 【アルゴリズム】 必要回数 D = Σ|A-B| が K 以下、かつ K - D が偶数なら可能（余りは +1,-1 で打ち消す）
// 【計算量】       O(N)
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - i64::abs() … (a - b).abs()。Py の abs() は関数、Rust はメソッド
//   - 早期 return で分岐を浅くする書き方
//   - % の符号: Rust の % は被除数の符号に従う（-3 % 2 == -1）。Py は常に非負（-3 % 2 == 1）
//
// 【Pythonで書くと（考え方の対応）】
//   d = sum(abs(x - y) for x, y in zip(a, b))
//   print('Yes' if d <= k and (k - d) % 2 == 0 else 'No')
//
// 【注意・改善ポイント】
//   ! 負の数の % は Py と結果が違う！ ここは diff, k とも非負なので問題ないが、負になりうる場合は
//     x.rem_euclid(m) を使う（Py の x % m と同じ非負の余りになる）。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    // Step #2. Calculate the absolute difference sum
    // 型は後の diff > k（i64）から i64 と推論。明示するなら let mut diff: i64 = 0;
    let mut diff = 0;
    for i in 0..n {
        // Py: diff += abs(a[i] - b[i])。別解: let diff: i64 = a.iter().zip(&b).map(|(x, y)| (x - y).abs()).sum();
        diff += (a[i] - b[i]).abs();
    }

    // Step #3. Check if the total difference exceeds K
    if diff > k {
        println!("No");
        return;
    }

    // Step #4. Check parity
    // 偶奇が一致すれば、余った手数を +1/-1 の往復で消費できる。
    if diff % 2 != k % 2 {
        println!("No");
        return;
    }

    // Step #5. Output Yes if both conditions are satisfied
    println!("Yes");
}
