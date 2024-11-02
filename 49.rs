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
        edges: [(i64, usize, usize); m],
    }

    // Sort edges by cost
    let mut sorted_edges = edges.clone();
    sorted_edges.sort_by_key(|&(c, _, _)| c);

    let mut uf = UnionFind::new(n + 2);
    let mut total_cost = 0;
    let mut edge_count = 0;

    // Kruskal's Algorithm
    for (cost, l, r) in sorted_edges {
        let u = l - 1;
        let v = r;
        if !uf.same(u, v) {
            uf.unite(u, v);
            total_cost += cost;
            edge_count += 1;
        }
    }

    // Check if all nodes are connected
    if edge_count != n {
        println!("-1");
    } else {
        println!("{}", total_cost);
    }
}
