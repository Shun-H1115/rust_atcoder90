// ==========================================================================
// 典型90 #044  Shift and Swapping  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ar
// ==========================================================================
// 【アルゴリズム】 配列を実際には回さず「ずれ量 shifts」だけ管理（添字を (x + shifts) % n で読み替え）
// 【計算量】       O(N + Q)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - match t { 1 => {...}, 2 => {...}, _ => {} } … Py 3.10 の match/case、または if-elif
//   - _ => {} … それ以外（Rust の match は全パターン網羅が必須なので必要）
//   - a.swap(i, j) … Py: a[i], a[j] = a[j], a[i]
//
// 【Pythonで書くと（考え方の対応）】
//   shift = 0
//   for t, x, y in queries:
//       if t == 1: i, j = (x-1+shift) % n, (y-1+shift) % n; a[i], a[j] = a[j], a[i]
//       elif t == 2: shift = (shift - 1) % n
//       else: print(a[(x-1+shift) % n])
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i32; n],
    }

    let mut shifts = 0;

    for _ in 0..q {
        input! {
            t: i32,
            x: usize,
            y: usize,
        }

        // match は式でもあり、全パターンを網羅しないとコンパイルエラー（取りこぼし防止）。
        match t {
            1 => {
                // Perform a swap
                let xi = (x - 1 + shifts) % n;
                let yi = (y - 1 + shifts) % n;
                a.swap(xi, yi);
            }
            2 => {
                // Shift operation
                // (shifts - 1) % n と書きたいが usize は負になれないので + n してから引く。
                // Py の (shift - 1) % n は負でも正しく動くが、Rust では書き方を工夫する。
                shifts = (shifts + n - 1) % n;
            }
            3 => {
                // Print the result
                let xi = (x - 1 + shifts) % n;
                println!("{}", a[xi]);
            }
            // _ はワイルドカード。t は 1〜3 しか来ないが、型上は i32 全体なので必要。
            _ => {}
        }
    }
}
