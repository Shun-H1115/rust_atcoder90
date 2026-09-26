// ==========================================================================
// 典型90 #001  Yokan Party  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_a
// ==========================================================================
// 【アルゴリズム】 答えで二分探索（「最小ピースの長さ >= m にできるか？」を判定して境界を探す）
// 【計算量】       O(N log L)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - proconio::input! マクロ … Pythonの input().split() 地獄から解放される標準的な入力方法
//   - スライス引数 &[i64] … Vec<i64> を借用で渡す。Pyのリスト渡しと違い「読むだけ」と型で宣言している
//   - 関数の最後の式 `cnt >= k` に ; がない → それが戻り値（return 省略）
//   - i64 と usize の使い分け: 添字は usize、値や負になりうる計算は i64
//
// 【Pythonで書くと（考え方の対応）】
//   def solve(m):
//       cnt, pre = 0, 0
//       for x in a:
//           if x - pre >= m and L - x >= m:
//               cnt += 1; pre = x
//       return cnt >= K
//   left, right = 0, L + 1
//   while right - left > 1:
//       mid = (left + right) // 2
//       if solve(mid): left = mid
//       else: right = mid
//   print(left)
// ==========================================================================

use proconio::input;

// a: &[i64] は「i64のスライスを借用」。Vec<i64> の &a を渡すと自動で &[i64] になる。
// Py: def solve(m, n, k, l, a): ...（型なし・常に参照渡し）
fn solve(m: i64, n: usize, k: i64, l: i64, a: &[i64]) -> bool {
    // let mut … 再代入する変数には mut が必須。Pyは全変数が再代入可、Rustは既定で不変。
    let mut cnt = 0;
    let mut pre = 0;

    for i in 0..n {
        // a[i] - pre >= m : 左側ピースの長さ。l - a[i] >= m : 残りの右端ピースも m 以上残るか。
        if a[i] - pre >= m && l - a[i] >= m {
            cnt += 1;
            pre = a[i];
        }
    }

    // ; が無い最後の式が戻り値。Py: return cnt >= k
    cnt >= k
}

fn main() {
    // Step #1. Input
    // input! { 変数名: 型, ... } で宣言と読み込みを同時に行う。
    // [i64; n] は「i64をn個」＝ Py: list(map(int, input().split()))
    input! {
        n: usize,
        l: i64,
        k: i64,
        a: [i64; n],
    }

    // Step #2. Binary search
    // 型注釈なしでも right(i64) との演算から left も i64 に推論される。
    let mut left = 0;
    let mut right = l + 1;

    // めぐる式二分探索: left は常に OK、right は常に NG を保つ。
    while right - left > 1 {
        // (left+right)/2 と同じだが、オーバーフロー対策の定番の書き方。
        let mid = left + (right - left) / 2;
        // &a で借用して渡す。a の所有権は main に残るので、ループで何度でも渡せる。
        if solve(mid, n, k, l, &a) {
            left = mid;
        } else {
            right = mid;
        }
    }

    // println!("{}", x) … Py: print(x)。{} はDisplayトレイトによる整形。
    println!("{}", left);
}
