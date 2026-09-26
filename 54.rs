// ==========================================================================
// 典型90 #054  Takahashi Number  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bb
// ==========================================================================
// 【アルゴリズム】 超頂点つきBFS（各論文グループを追加頂点にし、人↔論文 の二部グラフで BFS。距離/2 が答え）
// 【計算量】       O(N + M + ΣK)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - ループ内の input! で可変長の行（k 個）を読む
//   - vec![-2; n + m] … 未訪問を -2 にしておき、/2 したとき -1 になるようにする小技
//   - BFS の型推論: queue の要素型は push_back(0) と graph の添字利用から usize になる
//
// 【Pythonで書くと（考え方の対応）】
//   g = [[] for _ in range(n + m)]
//   for i in range(m):
//       k, *nodes = map(int, input().split())
//       for v in nodes: g[v-1].append(n+i); g[n+i].append(v-1)
//   # BFS で dist を求め dist[i] // 2（未到達は -1）
// ==========================================================================

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, // Number of nodes
        m: usize, // Number of groups
    }

    // Initialize the graph with N+M nodes
    // N 人 + M 論文の頂点を用意。論文頂点が「全員を結ぶハブ」になる（辺の数を ΣK に抑える工夫）。
    let mut graph = vec![vec![]; n + m];

    for i in 0..m {
        input! {
            k: usize, // Number of nodes in the group
            // [usize; k] … 直前に読んだ k を使って可変長を読める。proconio の便利な点。
            nodes: [usize; k], // List of nodes in the group
        }

        // Connect each node in the group to the group node (N + i)
        for &node in &nodes {
            graph[node - 1].push(n + i);
            graph[n + i].push(node - 1);
        }
    }

    // Initialize the distance vector with -2 (unvisited)
    // -2 / 2 = -1 なので、未到達の出力がそのまま -1 になる。要素型は i32。
    let mut dist = vec![-2; n + m];
    dist[0] = 0;

    // BFS initialization
    let mut queue = VecDeque::new();
    queue.push_back(0);

    // Perform BFS
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if dist[v] == -2 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }

    // Output the result for each of the first N nodes
    for i in 0..n {
        // 人→論文→人 で2歩なので /2。Rust の整数除算は 0 方向切り捨てなので -2/2 = -1 で OK。
        println!("{}", dist[i] / 2);
    }
}
