// ==========================================================================
// 典型90 #050  Stair Jump  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ax
// ==========================================================================
// 【アルゴリズム】 DP（dp[i] = dp[i-1] + dp[i-L]）
// 【計算量】       O(N)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - 関数内 const … main の中で宣言した定数（スコープは関数内）
//   - if i < l で分岐して i - l のアンダーフローを回避
//
// 【Pythonで書くと（考え方の対応）】
//   dp = [1] + [0]*n
//   for i in range(1, n+1):
//       dp[i] = dp[i-1] + (dp[i-L] if i >= L else 0)
//       dp[i] %= MOD
// ==========================================================================

use proconio::input;

fn main() {
    // const は関数内にも書ける。i32 の MOD は足し算1回なら溢れない（#042 の注意参照）。
    const MOD: i32 = 1_000_000_007;
    
    // Step #1: Input
    input! {
        n: usize,
        l: usize,
    }

    // Step #2: Dynamic Programming Initialization
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        // i - l は usize。i < l のとき計算すると panic するので必ず先に分岐する。
        if i < l {
            dp[i] = dp[i - 1];
        } else {
            dp[i] = (dp[i - 1] + dp[i - l]) % MOD;
        }
    }

    // Step #3: Output the result
    println!("{}", dp[n]);
}
