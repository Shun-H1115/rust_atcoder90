// ==========================================================================
// 典型90 #062  Paint All  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bj
// ==========================================================================
// 【アルゴリズム】 逆から考える＋BFS（最後に塗れるボールから逆向きに辿り、得られた順を反転）
// 【計算量】       O(N)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - for (i, (a, b)) in ab.iter().enumerate() … 添字とタプルを同時に分解（a, b は &usize）
//   - *a … 参照外しで値を取り出す
//   - BufWriter + writeln! の高速出力
//
// 【Pythonで書くと（考え方の対応）】
//   # ボール i は、a_i か b_i が既に白（= 自分で決まった）なら塗れる
//   # 逆操作: 「自分自身を指す」ボールから始め、関係するボールを BFS で辿る
//   # 辿った順の逆が塗る順番
// ==========================================================================

use proconio::input;
use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    // Step 1: Input
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut usable = vec![false; n + 1];
    let mut g = vec![vec![]; n + 1];
    let mut q = VecDeque::new();

    // ab.iter() なので a, b は &usize。for (i, &(a, b)) in ab.iter().enumerate() と書けば値で受け取れる。
    for (i, (a, b)) in ab.iter().enumerate() {
        let idx = i + 1;
        // g[*a] … 参照外しして usize として添字に。
        g[*a].push(idx);
        g[*b].push(idx);

        // Mark nodes directly usable
        // a_i == i または b_i == i のボールは最初に（逆順では最後に）塗れる。
        if *a == idx || *b == idx {
            usable[idx] = true;
            q.push_back(idx);
        }
    }

    // Step 2: Process nodes in a queue
    // vec は変数名。マクロ vec! とは別なので衝突しないが、order などの名前の方が読みやすい。
    let mut vec = Vec::new();
    while let Some(pos) = q.pop_front() {
        vec.push(pos);
        for &i in &g[pos] {
            if !usable[i] {
                usable[i] = true;
                q.push_back(i);
            }
        }
    }

    // Step 3: Output result
    // 逆操作で得た順序を反転 → 実際に塗る順。
    vec.reverse();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    // 全部辿れなければ不可能。
    if vec.len() != n {
        writeln!(handle, "-1").unwrap();
    } else {
        for v in vec {
            writeln!(handle, "{}", v).unwrap();
        }
    }
}
