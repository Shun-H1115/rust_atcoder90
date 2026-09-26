// ==========================================================================
// 典型90 #078  Easy Graph Problem  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bz
// ==========================================================================
// 【アルゴリズム】 各頂点について「自分より番号の小さい隣接頂点」がちょうど1つかを数える
// 【計算量】       O(N + M)
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - iter().filter(|&&j| j < i).count() … Py: sum(1 for j in g[i] if j < i)
//   - ループ内の input! で辺を1本ずつ読む
//
// 【Pythonで書くと（考え方の対応）】
//   print(sum(1 for i in range(n) if sum(1 for j in g[i] if j < i) == 1))
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        n: usize, // Number of vertices
        m: usize, // Number of edges
    }
    
    // Vec::new() の Vec … 要素型は push(b - 1) から usize と推論。
    let mut g = vec![Vec::new(); n]; // Adjacency list

    // Step #2. Read edges
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        g[a - 1].push(b - 1); // Convert to 0-based index
        g[b - 1].push(a - 1); // Undirected graph
    }

    // Step #3. Count vertices with exactly one smaller neighbor
    let mut answer = 0;
    for i in 0..n {
        // |&&j| … iter() の要素 &usize、filter の引数はその参照 &&usize。& を2回外して usize に（#030 参照）。
        let cnt = g[i].iter().filter(|&&j| j < i).count(); // Count neighbors smaller than i
        if cnt == 1 {
            answer += 1; // Increment answer if exactly one smaller neighbor
        }
    }

    // Step #4. Output the result
    println!("{}", answer);
}
