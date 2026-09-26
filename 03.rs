// ==========================================================================
// 典型90 #003  Longest Circular Road  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_c
// ==========================================================================
// 【アルゴリズム】 木の直径（BFSを2回: 任意の点→最遠点A、A→最遠点B、AB間の距離が直径）
// 【計算量】       O(N)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - VecDeque … Py の collections.deque。push_back / pop_front
//   - Option<usize> … Py の None 許容値。Some(x) / None を型で区別する
//   - while let Some(v) = queue.pop_front() … Py: while q: v = q.popleft()
//   - if let Some(d) = dist[i] … None でないときだけ中身を取り出す
//   - 隣接リスト vec![vec![]; n+1] … Py: [[] for _ in range(n+1)]
//   - タプルを返す関数 -> (usize, Vec<Option<usize>>)
//
// 【Pythonで書くと（考え方の対応）】
//   from collections import deque
//   def bfs(s):
//       dist = [None] * len(graph); dist[s] = 0
//       q = deque([s])
//       while q:
//           v = q.popleft()
//           for u in graph[v]:
//               if dist[u] is None:
//                   dist[u] = dist[v] + 1; q.append(u)
//       far = max(range(len(dist)), key=lambda i: dist[i] or -1)
//       return far, dist
// ==========================================================================

use proconio::input;
use std::collections::VecDeque;

// graph: &Vec<Vec<usize>> … 借用で受け取る（&[Vec<usize>] と書くとより汎用的）。
fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> (usize, Vec<Option<usize>>) {
    let n = graph.len();
    // vec![None; n] … 要素型は後の Some(0) から Option<usize> と推論される。
    // -1 を「未訪問」にする Py 流も使えるが、usize は負を持てないので Option が Rust らしい書き方。
    let mut dist = vec![None; n];  // Option<usize>に変更
    let mut queue = VecDeque::new();
    dist[start] = Some(0);
    queue.push_back(start);

    // while let: pop_front() が Some を返す間ループ。キューが空になり None で終了。
    while let Some(v) = queue.pop_front() {
        // dist[v] は必ず Some のはずだが、型上は Option なので if let で取り出している。
        // dist[v].unwrap() と書いても良い（None なら panic）。
        if let Some(v_dist) = dist[v] {
            // for &u in &graph[v] … &graph[v] を借用して走査、&u パターンで参照を外して usize をコピーで受け取る。
            for &u in &graph[v] {
                // is_none() … Py: dist[u] is None
                if dist[u].is_none() {
                    dist[u] = Some(v_dist + 1);
                    queue.push_back(u);
                }
            }
        }
    }

    let mut max_dist = 0;
    let mut farthest_node = start;
    for i in 0..n {
        if let Some(d) = dist[i] {
            if d > max_dist {
                max_dist = d;
                farthest_node = i;
            }
        }
    }

    // (farthest_node, dist) … タプルで2値返し。dist の所有権は呼び出し元へ移動（move）。
    (farthest_node, dist)
}

fn main() {
    input! {
        n: usize,
        // [(usize, usize); n - 1] … 1行に2数の行を n-1 行読む。Py: [tuple(map(int, input().split())) for _ in range(n-1)]
        edges: [(usize, usize); n - 1],
    }

    // 頂点番号が1始まりなので n+1 個確保（0番は未使用）。
    let mut graph = vec![vec![]; n + 1];
    // for (a, b) in edges … edges の所有権をループに渡す（以後 edges は使えない）。
    // 使い続けたいなら for &(a, b) in &edges。
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    // 1回目のBFSで最も遠い点を見つける
    // (farthest, _) … 不要な値は _ で捨てる。Py と同じ。
    let (farthest, _) = bfs(1, &graph);

    // 2回目のBFSでその点から最も遠い点を見つける
    let (other_farthest, dist) = bfs(farthest, &graph);

    // 木の直径（最長距離）
    if let Some(diameter) = dist[other_farthest] {
        // 直径 + 1 が最大スコア
        println!("{}", diameter + 1);
    }
}
