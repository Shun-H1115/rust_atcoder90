use proconio::input;
use std::collections::VecDeque;

fn dfs(v: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, order: &mut Vec<usize>) {
    used[v] = true;
    for &next in &graph[v] {
        if !used[next] {
            dfs(next, graph, used, order);
        }
    }
    order.push(v);
}

fn dfs2(v: usize, rev_graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, component_size: &mut u64) {
    used[v] = true;
    *component_size += 1;
    for &next in &rev_graph[v] {
        if !used[next] {
            dfs2(next, rev_graph, used, component_size);
        }
    }
}

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n + 1];
    let mut rev_graph = vec![vec![]; n + 1];

    for (a, b) in edges {
        graph[a].push(b);
        rev_graph[b].push(a);
    }

    // Step #2. First DFS (on the original graph to get the finishing order)
    let mut used = vec![false; n + 1];
    let mut order = Vec::new();

    for i in 1..=n {
        if !used[i] {
            dfs(i, &graph, &mut used, &mut order);
        }
    }

    // Step #3. Second DFS (on the reversed graph in the order of the first DFS finishing times)
    let mut answer = 0u64;
    used.fill(false);
    order.reverse();

    for &v in &order {
        if !used[v] {
            let mut component_size = 0;
            dfs2(v, &rev_graph, &mut used, &mut component_size);
            answer += component_size * (component_size - 1) / 2;
        }
    }

    // Step #4. Output the answer
    println!("{}", answer);
}
