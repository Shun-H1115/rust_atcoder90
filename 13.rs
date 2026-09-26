// ==========================================================================
// 典型90 #013  Passing  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_m
// ==========================================================================
// 【アルゴリズム】 ダイクストラ法を2回（1→k と N→k の最短距離の和）
// 【計算量】       O((N+M) log N)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - BinaryHeap は「最大」ヒープ。Reverse で包むと最小ヒープになる（Py の heapq は最小ヒープ）
//   - while let Some(Reverse((d, pos))) = heap.pop() … パターンで Reverse とタプルを一気に分解
//   - const INF: i64 = 1 << 60; … 大きめの番兵。i64::MAX だと足し算で溢れるので 1<<60 が定番
//   - 重み付き隣接リスト Vec<Vec<(usize, i64)>>
//
// 【Pythonで書くと（考え方の対応）】
//   import heapq
//   def dijkstra(s):
//       dist = [INF]*(n+1); dist[s] = 0
//       pq = [(0, s)]
//       while pq:
//           d, v = heapq.heappop(pq)
//           if d > dist[v]: continue
//           for to, c in graph[v]:
//               if dist[to] > d + c:
//                   dist[to] = d + c; heapq.heappush(pq, (dist[to], to))
//       return dist
// ==========================================================================

use proconio::input;
// Reverse … 比較を逆転させるラッパー。BinaryHeap<Reverse<T>> で最小ヒープ。
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 1 << 60 ≒ 1.15e18。INF + コスト でも i64 上限(9.2e18)を超えない安全な値。
const INF: i64 = 1 << 60;

fn dijkstra(start: usize, n: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let mut dist = vec![INF; n + 1];
    // 型は push した値から BinaryHeap<Reverse<(i64, usize)>> と推論される。
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    // タプルは辞書式比較なので (距離, 頂点) の順に入れる（Py の heapq と同じ）。
    heap.push(Reverse((0, start)));

    // Py: while pq: d, pos = heappop(pq)。Reverse(...) もパターンとして分解できる。
    while let Some(Reverse((d, pos))) = heap.pop() {
        // 古いエントリ（既により短い距離で確定済み）はスキップ。ダイクストラの定番の枝刈り。
        if d > dist[pos] {
            continue;
        }

        // &(to, cost) … 隣接リストのタプルを分解してコピーで受け取る。
        for &(to, cost) in &graph[pos] {
            if dist[to] > dist[pos] + cost {
                dist[to] = dist[pos] + cost;
                heap.push(Reverse((dist[to], to)));
            }
        }
    }

    dist
}

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, i64); m],
    }

    let mut graph = vec![vec![]; n + 1];
    // for (a, b, c) in edges … edges を消費（move）して分解。無向グラフなので両方向に追加。
    for (a, b, c) in edges {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    // Step #2: Compute shortest path from node 1
    let dist1 = dijkstra(1, n, &graph);

    // Step #3: Compute shortest path from node N
    let distn = dijkstra(n, n, &graph);

    // Step #4: Output the answer for each node
    for i in 1..=n {
        // 頂点 k を経由する最短経路 = (1→k) + (k→N)。
        let answer = dist1[i] + distn[i];
        println!("{}", answer);
    }
}
