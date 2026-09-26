// ==========================================================================
// 典型90 #071  Fuzzy Priority  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bs
// ==========================================================================
// 【アルゴリズム】 トポロジカルソートの列挙（入次数0の頂点を1つずつ選ぶバックトラック。K 個見つけたら打ち切り）
// 【計算量】       O(K × (N + M) × N)
// 【学習計画】     第8週以降（★7。バックトラックの題材として通勤で読む）
//
// 【このファイルで覚えるRust文法】
//   - 関数の中に関数（fn dfs）を定義できる。ただしクロージャと違い外側の変数は捕獲できない → 全部引数で渡す
//   - 引数が多くなるのは Rust の再帰の宿命。struct にまとめて &mut self のメソッドにすると読みやすくなる
//   - stack.remove(i) / stack.insert(i, x) … Py の list.pop(i) / list.insert(i, x)
//   - perm.clone() … 現在の順列のコピーを答えに保存
//
// 【Pythonで書くと（考え方の対応）】
//   def dfs(depth):
//       if depth == n: ans.append(perm[:]); return True
//       for i in reversed(range(len(stack))):
//           if len(ans) == K: break
//           x = stack.pop(i)
//           for j in g[x]:
//               deg[j] -= 1
//               if deg[j] == 0: stack.append(j)
//           perm[depth] = x
//           if not dfs(depth + 1): return False
//           for j in g[x]:
//               if deg[j] == 0: stack.pop()
//               deg[j] += 1
//           stack.insert(i, x)
//       return True
//
// 【注意・改善ポイント】
//   ! 出力が最大 K × N 個。print! を大量に呼んでいるので BufWriter を使うと速い。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    // Initialize the graph and in-degree array
    let mut graph = vec![Vec::new(); n];
    // 要素型は dfs の引数 &mut Vec<usize> から usize と推論される。
    let mut in_degree = vec![0; n];

    // Read the edges and build the graph
    for _ in 0..m {
        input! {
            mut a: usize,
            mut b: usize,
        }
        a -= 1;
        b -= 1;
        graph[a].push(b);
        in_degree[b] += 1;
    }

    // Stack for nodes with in-degree 0 and other data structures
    let mut stack = Vec::new();
    // -1 で初期化 → i32。usize で持てば as 変換が要らない（-1 は不要なので vec![0; n] でよい）。
    let mut perm = vec![-1; n];
    let mut answer_list = Vec::new();

    // Start with nodes that have in-degree of 0
    for i in 0..n {
        if in_degree[i] == 0 {
            stack.push(i);
        }
    }

    // Recursive DFS function for generating topological orders
    // 関数内関数。main のローカル変数は見えないので、必要なものは全部引数で受け取る。
    fn dfs(
        depth: usize,
        n: usize,
        k: usize,
        graph: &Vec<Vec<usize>>,
        in_degree: &mut Vec<usize>,
        stack: &mut Vec<usize>,
        perm: &mut Vec<i32>,
        answer_list: &mut Vec<Vec<i32>>,
    ) -> bool {
        // If a topological order is complete, add it to the answer list
        if depth == n {
            // clone で現在の順列をコピーして保存（perm はこの後も書き換えるため）。
            answer_list.push(perm.clone());
            return true;
        }
        
        // 入次数0の頂点が無いのに N 個並べ終わっていない → 閉路があり順序が存在しない。
        if stack.is_empty() {
            return false;
        }
        
        for i in (0..stack.len()).rev() {
            if answer_list.len() == k {
                break;
            }

            // Process the current node
            let x = stack[i];
            // remove(i) は O(len)。stack が小さい前提。
            stack.remove(i);
            
            // Add children of x with in-degree 0 to the stack
            for &j in &graph[x] {
                in_degree[j] -= 1;
                if in_degree[j] == 0 {
                    stack.push(j);
                }
            }

            perm[depth] = x as i32;

            // Recursive call
            if !dfs(depth + 1, n, k, graph, in_degree, stack, perm, answer_list) {
                return false;
            }

            // Backtrack: revert changes
            // バックトラック: 追加した子を取り除き、入次数を戻す（追加した順と逆に戻す）。
            for &j in &graph[x] {
                if in_degree[j] == 0 {
                    stack.pop();
                }
                in_degree[j] += 1;
            }
            // 取り除いた位置に x を戻す。
            stack.insert(i, x);
        }
        true
    }

    // &graph は不変借用、残りは &mut。main の変数を一時的に dfs へ貸し出す。
    dfs(0, n, k, &graph, &mut in_degree, &mut stack, &mut perm, &mut answer_list);

    // Output the results or -1 if there aren't enough answers
    if answer_list.len() != k {
        println!("-1");
    } else {
        for v in answer_list {
            for (i, &x) in v.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                // x + 1 で 1-indexed に戻して出力。
                print!("{}", x + 1);
            }
            println!();
        }
    }
}
