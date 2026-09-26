// ==========================================================================
// 典型90 #049  Flip Digits 2  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_aw
// ==========================================================================
// 【アルゴリズム】 区間→累積XORの差分に言い換え → 頂点 L-1 と R を結ぶ辺とみなし最小全域木（クラスカル法）
// 【計算量】       O(M log M)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - UnionFind の再利用（#012 と同じ struct）→ ライブラリ化の効果を実感する問題
//   - sort_by_key(|&(c, _, _)| c) … コストでソート
//   - for (cost, l, r) in sorted_edges … 消費しながら分解
//
// 【Pythonで書くと（考え方の対応）】
//   edges.sort()
//   uf = UnionFind(n + 1); total = 0; cnt = 0
//   for c, l, r in edges:
//       if not uf.same(l - 1, r):
//           uf.unite(l - 1, r); total += c; cnt += 1
//   print(total if cnt == n else -1)
//
// 【注意・改善ポイント】
//   ! use std::cmp::Ordering と BinaryHeap は未使用（warning）。
//   ! edges.clone() は不要。input! で mut edges として受けて edges.sort_by_key(...) すればコピーが要らない。
// ==========================================================================

use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct UnionFind {
    par: Vec<i32>,
}

impl UnionFind {
    fn new(sz: usize) -> Self {
        UnionFind {
            par: vec![-1; sz],
        }
    }

    fn root(&mut self, pos: usize) -> usize {
        if self.par[pos] == -1 {
            return pos;
        }
        self.par[pos] = self.root(self.par[pos] as usize) as i32;
        self.par[pos] as usize
    }

    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u != root_v {
            self.par[root_u] = root_v as i32;
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        // (C, L, R) の順。コストを先頭にしておくと sort() だけでもコスト順になる。
        edges: [(i64, usize, usize); m],
    }

    // Sort edges by cost
    // clone でコピーを作っている。元の edges を後で使わないなら不要。
    let mut sorted_edges = edges.clone();
    sorted_edges.sort_by_key(|&(c, _, _)| c);

    let mut uf = UnionFind::new(n + 2);
    let mut total_cost = 0;
    let mut edge_count = 0;

    // Kruskal's Algorithm
    for (cost, l, r) in sorted_edges {
        let u = l - 1;
        // 区間 [L, R] の反転 = 累積XOR の差分列で位置 L-1 と R の2点を反転 → グラフの辺とみなせる。
        let v = r;
        if !uf.same(u, v) {
            uf.unite(u, v);
            total_cost += cost;
            edge_count += 1;
        }
    }

    // Check if all nodes are connected
    // 頂点 0..=n の n+1 個を全部つなぐには n 本の辺が必要（全域木）。
    if edge_count != n {
        println!("-1");
    } else {
        println!("{}", total_cost);
    }
}
