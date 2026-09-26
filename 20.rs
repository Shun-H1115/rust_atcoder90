// ==========================================================================
// 典型90 #020  Log Inequality  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_t
// ==========================================================================
// 【アルゴリズム】 整数で比較（log を使わず a < c^b を判定。オーバーフローを割り算で回避）
// 【計算量】       O(b)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - u64 … 符号なし64bit（最大 1.8e19）
//   - 1u64 … 型サフィックス付きリテラル
//   - return; … main から早期リターン（戻り値なし）
//   - オーバーフロー回避: e * c > a を e > a / c に変形（Py は多倍長なので不要だった処理）
//
// 【Pythonで書くと（考え方の対応）】
//   # Python なら多倍長整数なので一行
//   print('Yes' if a < c ** b else 'No')
//
// 【注意・改善ポイント】
//   ! Rust の整数は固定長。c^b を直接計算すると溢れる。
//     別解: e.checked_mul(c) は溢れると None を返すので match で判定できる（覚えておくと便利）。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let mut e = 1u64;
    for _ in 0..b {
        // e * c が a を超えるか？を掛け算せずに判定。e > a / c なら e * c > a が確定。
        if e > a / c {
            // If `e * c` would exceed `a`, we know `A < C^B` will be true
            println!("Yes");
            // return; … main を即終了。Py の sys.exit() に近いが、正常終了で後処理も走る。
            return;
        }
        // e *= c … ここに来るときは e * c <= a なので溢れない。
        e *= c;
    }

    if a < e {
        println!("Yes");
    } else {
        println!("No");
    }
}
