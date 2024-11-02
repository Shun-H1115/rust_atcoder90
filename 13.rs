use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i64 = 1 << 60;

fn dijkstra(start: usize, n: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let mut dist = vec![INF; n + 1];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, pos))) = heap.pop() {
        if d > dist[pos] {
            continue;
        }

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
        let answer = dist1[i] + distn[i];
        println!("{}", answer);
    }
}
