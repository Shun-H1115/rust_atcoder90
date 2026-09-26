// ==========================================================================
// 典型90 #021  Come Back in One Piece  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_u
// ==========================================================================
// 【アルゴリズム】 強連結成分分解（Kosaraju法: DFSの帰りがけ順 → 逆グラフで逆順にDFS）
// 【計算量】       O(N + M)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - 再帰関数に可変な状態を渡す: used: &mut Vec<bool>, order: &mut Vec<usize>
//   - （Py ならクロージャやグローバル変数で済むところ、Rust は借用を引数で明示的に回す）
//   - *component_size += 1 … &mut u64 の参照先を書き換える（参照外し *）
//   - used.fill(false) … 配列を一括初期化。Py: used = [False]*(n+1)
//   - order.reverse() … Py の list.reverse() と同じ（破壊的）
//
// 【Pythonで書くと（考え方の対応）】
//   def dfs(v):
//       used[v] = True
//       for nx in g[v]:
//           if not used[nx]: dfs(nx)
//       order.append(v)
//   # 逆グラフで order の逆順に dfs2 → 各成分サイズ c について c*(c-1)//2 を加算
//
// 【注意・改善ポイント】
//   ! N=1e5 の一直線グラフでは再帰が 1e5 段になる。AtCoder のジャッジは十分なスタックがあるが、
//     ローカル実行で stack overflow する場合は std::thread::Builder::new().stack_size(1<<28) で
//     大きなスタックのスレッドを立てて main 処理を走らせるのが Rust の定番の対処法。
// ==========================================================================

use proconio::input;
use std::collections::VecDeque;

// graph は読むだけなので &、used と order は書き換えるので &mut。
// 【重要】Rust では「可変参照は同時に1つだけ」。だから状態を関数引数でバケツリレーする。
fn dfs(v: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, order: &mut Vec<usize>) {
    used[v] = true;
    for &next in &graph[v] {
        if !used[next] {
            // 再帰呼び出しで used, order をそのまま渡す（既に &mut なので再度 &mut を付けない）。
            dfs(next, graph, used, order);
        }
    }
    // 帰りがけ順（子を全部訪問したあと）に記録。
    order.push(v);
}

fn dfs2(v: usize, rev_graph: &Vec<Vec<usize>>, used: &mut Vec<bool>, component_size: &mut u64) {
    used[v] = true;
    // *component_size … 参照の中身を書き換える。Py の nonlocal cnt; cnt += 1 に相当。
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
        // 逆辺グラフ。SCC 分解の2回目の DFS で使う。
        rev_graph[b].push(a);
    }

    // Step #2. First DFS (on the original graph to get the finishing order)
    let mut used = vec![false; n + 1];
    let mut order = Vec::new();

    for i in 1..=n {
        if !used[i] {
            // &mut used … 可変借用を渡す。関数から戻ると借用は解除され、main で再び使える。
            dfs(i, &graph, &mut used, &mut order);
        }
    }

    // Step #3. Second DFS (on the reversed graph in the order of the first DFS finishing times)
    let mut answer = 0u64;
    // fill(false) で使い回し。新しく vec![false; n+1] を作っても良い。
    used.fill(false);
    order.reverse();

    for &v in &order {
        if !used[v] {
            // 0 の型は dfs2 の引数 &mut u64 から u64 と推論される。
            let mut component_size = 0;
            dfs2(v, &rev_graph, &mut used, &mut component_size);
            // 同じ SCC 内の頂点ペアは互いに行き来できる → C(size, 2) 組。
            answer += component_size * (component_size - 1) / 2;
        }
    }

    // Step #4. Output the answer
    println!("{}", answer);
}
