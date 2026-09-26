// ==========================================================================
// 典型90 #026  Independent Set on a Tree  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_z
// ==========================================================================
// 【アルゴリズム】 木の二部グラフ彩色（DFSで2色に塗り、多い方の色から N/2 個出力）
// 【計算量】       O(N)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - 再帰DFSで &mut Vec を引き回す（#021 と同じパターン）
//   - 3 - color … 1 と 2 を交互にする小技（Py でも同じ）
//   - 使っていない use（VecDeque）はコンパイラが warning を出す。warning は消す習慣を
//
// 【Pythonで書くと（考え方の対応）】
//   def dfs(v, c):
//       color[v] = c
//       for u in g[v]:
//           if color[u] == 0: dfs(u, 3 - c)
//   dfs(1, 1)
//   g1 = [i for i in range(1, n+1) if color[i] == 1]
//   g2 = [i for i in range(1, n+1) if color[i] == 2]
//   print(*(g1 if len(g1) >= len(g2) else g2)[:n//2])
//
// 【注意・改善ポイント】
//   ! 木が一直線だと再帰が N=1e5 段になる（#021 の注意を参照）。BFS で塗れば再帰を避けられる
//     （VecDeque を import しているので、BFS 版に書き換える練習にちょうど良い）。
// ==========================================================================

use proconio::input;
// （未使用）VecDeque は使っていないので warning が出る。BFS 版にするなら使う。
use std::collections::VecDeque;

// graph は読むだけ（&）、colors は書き換える（&mut）。
fn dfs(pos: usize, color: i32, graph: &Vec<Vec<usize>>, colors: &mut Vec<i32>) {
    colors[pos] = color;
    for &neighbor in &graph[pos] {
        if colors[neighbor] == 0 {
            dfs(neighbor, 3 - color, graph, colors); // Alternate between 1 and 2
        }
    }
}

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        edges: [(usize, usize); n - 1],
    }

    // Initialize the graph and colors vector
    // vec![Vec::new(); n + 1] は vec![vec![]; n + 1] と同じ。
    let mut graph = vec![Vec::new(); n + 1];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut colors = vec![0; n + 1];

    // Step #2: Graph Coloring
    // &mut colors … 可変借用で渡す。dfs が戻れば main で colors を普通に読める。
    dfs(1, 1, &graph, &mut colors);

    // Step #3: Separate nodes by color
    let mut group1 = Vec::new();
    let mut group2 = Vec::new();
    for i in 1..=n {
        if colors[i] == 1 {
            group1.push(i);
        } else if colors[i] == 2 {
            group2.push(i);
        }
    }

    // Step #4: Output half of the larger group
    // len() は usize。二部グラフの片側は必ず N/2 以上ある（鳩の巣原理）。
    if group1.len() >= group2.len() {
        // 出力部分は重複しているので、let g = if ... { &group1 } else { &group2 }; として1つにまとめられる。
        // さらに g[..n/2].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ") で Py の print(*...) 相当。
        for i in 0..n / 2 {
            if i > 0 {
                print!(" ");
            }
            print!("{}", group1[i]);
        }
    } else {
        for i in 0..n / 2 {
            if i > 0 {
                print!(" ");
            }
            print!("{}", group2[i]);
        }
    }
    println!();
}
