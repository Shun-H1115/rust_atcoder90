use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, // Number of nodes
        m: usize, // Number of groups
    }

    // Initialize the graph with N+M nodes
    let mut graph = vec![vec![]; n + m];

    for i in 0..m {
        input! {
            k: usize, // Number of nodes in the group
            nodes: [usize; k], // List of nodes in the group
        }

        // Connect each node in the group to the group node (N + i)
        for &node in &nodes {
            graph[node - 1].push(n + i);
            graph[n + i].push(node - 1);
        }
    }

    // Initialize the distance vector with -2 (unvisited)
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
        println!("{}", dist[i] / 2);
    }
}
