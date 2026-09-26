// ==========================================================================
// 典型90 #074  ABC String 2  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bv
// ==========================================================================
// 【アルゴリズム】 不変量／数学的考察（位置 i の文字を a=0, b=1, c=2 とみなした Σ c_i × 2^i が操作回数の最大値）
// 【計算量】       O(N)
// 【学習計画】     第7週（★6。コードは短いが考察が本体。解説を読んで「なぜ 2^i か」を理解する）
//
// 【このファイルで覚えるRust文法】
//   - s.chars().enumerate() … (添字, 文字)
//   - 1 << i の型は左辺 answer（i64）から推論される。i32 だと i >= 31 で溢れるので型注釈が効いている
//   - 未使用変数 n は _n にすると warning が消える
//
// 【Pythonで書くと（考え方の対応）】
//   print(sum({'a': 0, 'b': 1, 'c': 2}[ch] << i for i, ch in enumerate(S)))
// ==========================================================================

use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        s: String,
    }

    // Initialize answer to zero
    // 型注釈 : i64 が重要。これが無いと answer と 1 << i が i32 と推論され、N が大きいと溢れる。
    let mut answer: i64 = 0;

    // Calculate answer by iterating over each character in the string
    for (i, ch) in s.chars().enumerate() {
        if ch == 'b' {
            // If character is 'b', add 1 * 2^i to the answer
            // 1 << i … i は usize（シフト量は何の整数型でもよい）、結果の型は i64。
            answer += 1 << i;
        } else if ch == 'c' {
            // If character is 'c', add 2 * 2^i to the answer
            // 2 << i は 2 × 2^i。
            answer += 2 << i;
        }
    }

    // Output the final answer
    println!("{}", answer);
}
