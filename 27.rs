// ==========================================================================
// 典型90 #027  Sign Up Requests  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_aa
// ==========================================================================
// 【アルゴリズム】 集合で初出判定（HashSet / HashMap）
// 【計算量】       O(N)（平均）
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - [String; n] … 文字列 n 行を Vec<String> で受け取る
//   - HashMap::new() / contains_key / insert … Py の dict
//   - （この用途なら HashSet が自然）HashSet::insert は「新規なら true」を返すので1行で書ける
//   - s.iter().enumerate() … Py の enumerate(s)
//
// 【Pythonで書くと（考え方の対応）】
//   seen = set()
//   for i, w in enumerate(S, 1):
//       if w not in seen:
//           seen.add(w); print(i)
// ==========================================================================

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    // キーと値の型は insert から推論される（ここでは HashMap<&String, bool>）。
    // より良い書き方: let mut set = HashSet::new(); if set.insert(word) { println!(...) }
    let mut map = HashMap::new();

    // s.iter() は借用なので word は &String。map にも参照を入れているので s より長生きできない（ライフタイム）。
    for (i, word) in s.iter().enumerate() {
        // !map.contains_key(word) … Py: word not in d
        if !map.contains_key(word) {
            map.insert(word, true);
            println!("{}", i + 1);
        }
    }
}
