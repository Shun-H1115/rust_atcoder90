// ==========================================================================
// 典型90 #042  Multiple of 9  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ap
// ==========================================================================
// 【アルゴリズム】 DP（dp[i] = 桁和 i になる並べ方。各桁 1〜9。K が 9 の倍数でなければ 0）
// 【計算量】       O(9K)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - saturating_sub … 0 未満にならない引き算。Py の max(0, i - 9)
//   - MOD を i32 にしているので dp は i32。dp + dp が 2^31 を超えないかの確認が必要（今回は MOD*2 < 2^31 で OK）
//
// 【Pythonで書くと（考え方の対応）】
//   if K % 9: print(0)
//   else:
//       dp = [1] + [0]*K
//       for i in range(1, K+1):
//           dp[i] = sum(dp[max(0, i-9):i]) % MOD
//       print(dp[K])
//
// 【注意・改善ポイント】
//   ! i32 の MOD 演算はギリギリ（(MOD-1)*2 ≒ 2.0e9 < 2.147e9）。掛け算が入ると即溢れるので、
//     MOD 演算は i64 で統一するのが安全。
// ==========================================================================

use proconio::input;
const MOD: i32 = 1_000_000_007;

fn main() {
    input! {
        k: usize,
    }

    if k % 9 == 0 {
        // dp の型は MOD（i32）との演算から i32 と推論される。
        let mut dp = vec![0; k + 1];
        dp[0] = 1;

        for i in 1..=k {
            // i.saturating_sub(9) … i < 9 なら 0。usize のアンダーフローを避ける定番メソッド。
            for j in (i.saturating_sub(9))..i {
                dp[i] = (dp[i] + dp[j]) % MOD;
            }
        }

        println!("{}", dp[k]);
    } else {
        println!("0");
    }
}
