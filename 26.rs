use proconio::input;
use std::collections::VecDeque;

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
    let mut graph = vec![Vec::new(); n + 1];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut colors = vec![0; n + 1];

    // Step #2: Graph Coloring
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
    if group1.len() >= group2.len() {
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
