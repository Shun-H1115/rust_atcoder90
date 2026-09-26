// ==========================================================================
// 典型90 #039  Tree Distance  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_am
// ==========================================================================
// 【アルゴリズム】 主客転倒（各辺が何個のペアの経路に使われるか = 片側の頂点数 × 反対側の頂点数）
// 【計算量】       O(N)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - pre: isize で「親なし = -1」を表現（usize は負を持てないため）
//   - 部分木サイズ DP を再帰で計算
//   - dp[a].min(dp[b]) … 辺の子側の部分木サイズ（小さい方が子）
//
// 【Pythonで書くと（考え方の対応）】
//   def dfs(v, p):
//       size[v] = 1
//       for u in g[v]:
//           if u != p: dfs(u, v); size[v] += size[u]
//   dfs(1, -1)
//   print(sum(min(size[a], size[b]) * (n - min(size[a], size[b])) for a, b in edges))
//
// 【注意・改善ポイント】
//   ! 親を isize で持つ代わりに、根の親を 0（未使用の頂点番号）にすれば usize のままで書ける。
//     1-indexed のグラフでは dfs(1, 0, ...) とするのがよくある手。
// ==========================================================================

use proconio::input;

// pre: isize … 根の親として -1 を渡したいため符号付き。
fn dfs(pos: usize, pre: isize, dp: &mut Vec<i64>, graph: &Vec<Vec<usize>>) {
    dp[pos] = 1;
    for &next in &graph[pos] {
        // next as isize == pre … 型を揃えて比較。Rust は usize と isize を直接比較できない。
        if next as isize == pre {
            continue;
        }
        dfs(next, pos as isize, dp, graph);
        // 子の部分木サイズを足し上げる（帰りがけに計算）。
        dp[pos] += dp[next];
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n - 1],
    }

    let mut graph = vec![vec![]; n + 1];
    // &edges で借用して走査。edges は後でもう一度使うので move してはいけない。
    for &(a, b) in &edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    // 要素型は後の r * (n as i64 - r) から i64 と推論される。
    let mut dp = vec![0; n + 1];
    dfs(1, -1, &mut dp, &graph);

    let mut answer: i64 = 0;
    for &(a, b) in &edges {
        let r = dp[a].min(dp[b]);
        // n as i64 - r … usize の n を i64 に揃えて計算。
        answer += r * (n as i64 - r);
    }

    println!("{}", answer);
}
