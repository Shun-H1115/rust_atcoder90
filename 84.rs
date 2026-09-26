// ==========================================================================
// 典型90 #084  There are two types of characters  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cf
// ==========================================================================
// 【アルゴリズム】 余事象＋ランレングス圧縮（全区間数 - 1種類の文字だけの区間数）
// 【計算量】       O(N)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - s.as_bytes()[i] … &[u8] として添字アクセス（ASCII のとき高速で手軽）
//   - ランレングス圧縮を手書き（Py: itertools.groupby）
//   - for (_, count) in vec … タプルの不要な要素は _
//
// 【Pythonで書くと（考え方の対応）】
//   from itertools import groupby
//   lens = [len(list(g)) for _, g in groupby(S)]
//   same = sum(k * (k + 1) // 2 for k in lens)
//   print(N*(N+1)//2 - same)
//
// 【注意・改善ポイント】
//   ! s.as_bytes() を毎回呼ぶのは冗長。let s = s.as_bytes(); とシャドーイングしておけば s[i] で書ける。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    // 連続する同じ文字の数。型は後の n * (n + 1) / 2 - ret（usize）から usize に推論される。
    let mut cnt = 0;
    let mut vec = Vec::new();

    // Count consecutive characters
    for i in 0..s.len() {
        cnt += 1;
        // || は短絡評価。i == 末尾 なら右側（i + 1 のアクセス）は評価されない。
        if i == s.len() - 1 || s.as_bytes()[i] != s.as_bytes()[i + 1] {
            // (文字, 連続数) を記録。文字は使っていないので cnt だけ push してもよい。
            vec.push((s.as_bytes()[i] as char, cnt));
            cnt = 0;
        }
    }

    let mut ret = 0;
    for (_, count) in vec {
        // Calculate the sum of pairs in the consecutive characters
        // 同じ文字だけの区間数 = L(L+1)/2。
        ret += count * (count + 1) / 2;
    }

    // Total pairs minus the pairs of consecutive characters
    println!("{}", n * (n + 1) / 2 - ret);
}
