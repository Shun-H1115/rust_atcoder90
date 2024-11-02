use proconio::input;

fn dfs(pos: usize, pre: isize, dp: &mut Vec<i64>, graph: &Vec<Vec<usize>>) {
    dp[pos] = 1;
    for &next in &graph[pos] {
        if next as isize == pre {
            continue;
        }
        dfs(next, pos as isize, dp, graph);
        dp[pos] += dp[next];
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n - 1],
    }

    let mut graph = vec![vec![]; n + 1];
    for &(a, b) in &edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dp = vec![0; n + 1];
    dfs(1, -1, &mut dp, &graph);

    let mut answer: i64 = 0;
    for &(a, b) in &edges {
        let r = dp[a].min(dp[b]);
        answer += r * (n as i64 - r);
    }

    println!("{}", answer);
}
