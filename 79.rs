// ==========================================================================
// 典型90 #079  Two by Two  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ca
// ==========================================================================
// 【アルゴリズム】 貪欲法（左上から順に、そのマスを B に合わせるよう 2×2 に ±d を加える。最後に一致するか確認）
// 【計算量】       O(HW)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - a == b … Vec<Vec<i64>> 同士を == で丸ごと比較できる（Py の list 比較と同じ）
//   - h.saturating_sub(1) … h = 0 でも panic しない安全な h - 1
//   - input! の中で mut a: [[i64; w]; h] と2次元を可変で受け取る
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 0
//   for i in range(H-1):
//       for j in range(W-1):
//           d = B[i][j] - A[i][j]
//           for di, dj in ((0,0),(0,1),(1,0),(1,1)): A[i+di][j+dj] += d
//           ans += abs(d)
//   print('Yes\n' + str(ans) if A == B else 'No')
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut ans: i64 = 0;

    // Apply 2x2 adjustments left-to-right, top-to-bottom
    // saturating_sub … 0 - 1 のとき panic せず 0 を返す（制約上 h >= 2 なので念のための書き方）。
    for i in 0..h.saturating_sub(1) {
        for j in 0..w.saturating_sub(1) {
            // 左上マスを合わせる差分。以降このマスは触らない（貪欲の正当性）。
            let d = b[i][j] - a[i][j];
            a[i][j] += d;
            a[i][j + 1] += d;
            a[i + 1][j] += d;
            a[i + 1][j + 1] += d;
            ans += d.abs();
        }
    }

    // Vec の == は要素ごとの比較。PartialEq が実装されていれば多次元でもそのまま使える。
    if a == b {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
