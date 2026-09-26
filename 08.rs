// ==========================================================================
// 典型90 #008  AtCounter  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_h
// ==========================================================================
// 【アルゴリズム】 DP（dp[j] = "atcoder" の先頭 j 文字を作る部分列の個数）
// 【計算量】       O(7N)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - 文字列定数 let target = "atcoder"; は &str 型
//   - target.chars().nth(j).unwrap() … j 番目の文字。ただし毎回先頭から数えるので O(j)
//   - 未使用変数 n は警告が出る。_n と書くと警告を抑制できる
//
// 【Pythonで書くと（考え方の対応）】
//   dp = [1] + [0]*7
//   for ch in S:
//       for j in reversed(range(7)):
//           if ch == 'atcoder'[j]:
//               dp[j+1] = (dp[j+1] + dp[j]) % MOD
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        // n は使っていないが入力の読み飛ばしに必要。_n: usize と書けば unused 警告が消える。
        n: usize,
        s: String,
    }

    // 改善案: let target: Vec<char> = "atcoder".chars().collect(); にすれば target[j] で O(1) アクセス。
    // あるいは let target = b"atcoder"; （バイト列）で target[j] as char。
    let target = "atcoder";
    let mut dp = vec![0; 8];
    dp[0] = 1;

    for ch in s.chars() {
        // 後ろから更新するのは、同じ文字 ch で dp[j]→dp[j+1]→dp[j+2] と連鎖更新しないため（0-1ナップサックと同じ理屈）。
        for j in (0..7).rev() {
            // nth(j) は Option<char> を返すので unwrap() で取り出す。
            if ch == target.chars().nth(j).unwrap() {
                dp[j + 1] = (dp[j + 1] + dp[j]) % MOD;
            }
        }
    }

    // Output the answer
    println!("{}", dp[7]);
}
