// ==========================================================================
// 典型90 #002  Encyclopedia of Parentheses  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_b
// ==========================================================================
// 【アルゴリズム】 bit全探索（2^N 通りの括弧列を全部作って正しいものだけ出力）
// 【計算量】       O(2^N × N)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - 1 << n のビット演算は Py と同じ。ただし型に幅があるので n が大きいとオーバーフローに注意
//   - String::new() と push(char) … Py の文字列は不変だが Rust の String は可変バッファ
//   - &str と String の違い: String は所有する文字列、&str はその借用ビュー
//   - (0..n).rev() … Py: reversed(range(n))
//
// 【Pythonで書くと（考え方の対応）】
//   for i in range(1 << n):
//       cand = ''.join('(' if not (i >> j) & 1 else ')' for j in reversed(range(n)))
//       if is_valid(cand): print(cand)
// ==========================================================================

use proconio::input;

// 引数 &str: String でも文字列リテラルでも受け取れる汎用的な型。関数引数は &str が定石。
fn is_valid(s: &str) -> bool {
    let mut depth = 0;
    // s.chars() は1文字ずつ(char)のイテレータ。Py: for ch in s:
    for ch in s.chars() {
        // '(' はシングルクォート＝char型。"(" はダブルクォート＝&str型。Pyと違い区別される！
        if ch == '(' {
            depth += 1;
        } else {
            depth -= 1;
        }
        if depth < 0 {
            return false;
        }
    }
    // depth == 0 が戻り値（bool）。
    depth == 0
}

fn main() {
    input! {
        n: usize,
    }

    // 0..(1 << n) は 0 以上 2^n 未満。辞書順 '(' < ')' なので、上位ビットから並べれば自然に辞書順出力になる。
    for i in 0..(1 << n) {
        // ループ毎に新しい String を作る。Py: cand = []
        let mut candidate = String::new();
        // (0..n).rev() … 上位ビットから見る。
        for j in (0..n).rev() {
            if (i & (1 << j)) == 0 {
                // push は char を1つ追加。文字列を追加するなら push_str("..")。
                candidate.push('(');
            } else {
                candidate.push(')');
            }
        }

        // &candidate: String → &str に自動変換（Deref）されて渡る。
        if is_valid(&candidate) {
            println!("{}", candidate);
        }
    }
}
