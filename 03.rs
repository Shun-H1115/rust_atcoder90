use proconio::input;
use std::collections::VecDeque;

fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> (usize, Vec<Option<usize>>) {
    let n = graph.len();
    let mut dist = vec![None; n];  // Option<usize>に変更
    let mut queue = VecDeque::new();
    dist[start] = Some(0);
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        if let Some(v_dist) = dist[v] {
            for &u in &graph[v] {
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

    (farthest_node, dist)
}

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n - 1],
    }

    let mut graph = vec![vec![]; n + 1];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    // 1回目のBFSで最も遠い点を見つける
    let (farthest, _) = bfs(1, &graph);

    // 2回目のBFSでその点から最も遠い点を見つける
    let (other_farthest, dist) = bfs(farthest, &graph);

    // 木の直径（最長距離）
    if let Some(diameter) = dist[other_farthest] {
        // 直径 + 1 が最大スコア
        println!("{}", diameter + 1);
    }
}
