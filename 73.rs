// ==========================================================================
// 典型90 #073 We Need Both a and b（★5） 
// 【修正点】元ファイルは2つの別バージョンが途中で混ざっており、fn main / use / const が
//   二重定義されてコンパイルできなかった。正しく動く方（入力 [char; n]、非再帰DFS）の
//   1本に整理し、意味が不明確だったコメントを書き直した。
//
// 【DPの定義】dp[v][t] = v の部分木内の辺の切り方のうち、
//     v を含む連結成分（まだ親側とつながりうる）の状態が t であるもの。
//     それ以外の「切り離された」成分はすべて a と b の両方を含んでいること。
//     t = 0: a のみ / t = 1: b のみ / t = 2: a と b の両方
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        c_in: [char; n],              // 空白区切りの n 文字
        edges: [(usize, usize); n - 1],
    }

    // 1-indexed
    let mut c = vec!['?'; n + 1];
    for i in 1..=n {
        c[i] = c_in[i - 1];
    }

    let mut g = vec![Vec::<usize>::new(); n + 1];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }

    // 非再帰DFSで親と訪問順を求める（深い木でもスタックオーバーフローしない）
    let root = 1usize;
    let mut parent = vec![usize::MAX; n + 1];
    parent[root] = root;
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![root];
    while let Some(v) = stack.pop() {
        order.push(v);
        for &to in &g[v] {
            if parent[to] != usize::MAX {
                continue;
            }
            parent[to] = v;
            stack.push(to);
        }
    }

    let mut dp = vec![[0i64; 3]; n + 1];

    // 訪問順の逆 = 子が必ず親より先に処理される
    for &v in order.iter().rev() {
        // val1: v の成分が v と同じ文字だけのままである切り方の数
        // val2: v の成分が何でもよい（同じ文字のみ or 両方）切り方の数
        let mut val1: i64 = 1;
        let mut val2: i64 = 1;

        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            let (d0, d1, d2) = (dp[to][0], dp[to][1], dp[to][2]);

            // 子と同じ文字だけの成分に保つ: 辺をつなぐなら子は同じ文字のみ、
            // 辺を切るなら子の成分は両方を含んでいる必要がある（d2）
            let same = if c[v] == 'a' { d0 } else { d1 };
            let t1 = (same + d2) % MOD;

            // 何でもよい: 辺をつなぐ（d0 + d1 + d2）＋ 辺を切る（d2 のみ可）
            let t2 = (d0 + d1 + 2 * d2) % MOD;

            val1 = val1 * t1 % MOD;
            val2 = val2 * t2 % MOD;
        }

        let idx = if c[v] == 'a' { 0 } else { 1 };
        dp[v][idx] = val1;
        // 両方を含む = 全体 - 同じ文字のみ
        dp[v][2] = (val2 - val1).rem_euclid(MOD);
    }

    // 根の成分も a と b の両方を含む必要がある
    println!("{}", dp[root][2]);
}
