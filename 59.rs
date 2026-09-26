// ==========================================================================
// 典型90 #059  Many Graph Queries  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bg
// ==========================================================================
// 【アルゴリズム】 bitset 高速化（クエリを64個ずつまとめ、u64 の各ビットを1クエリとして DAG 上で到達可能性を一括伝播）
// 【計算量】       O((N + M) × Q / 64)
// 【学習計画】     第8週（★7 実装推奨5問の1つ。u64 ビット演算の総仕上げ）
//
// 【このファイルで覚えるRust文法】
//   - 高速出力: BufWriter::new(stdout.lock()) + write!/writeln! … 大量出力の定番（#004 などでも使える）
//   - use std::io::Write; が無いと write! マクロが使えない
//   - u64 を 64 個の bool の束として使う（Py の int もビット演算できるが、Rust は1命令で処理される）
//   - (q + 63) / 64 … 切り上げ除算。Py: -(-q // 64)
//
// 【Pythonで書くと（考え方の対応）】
//   for s in range(0, Q, 64):
//       dp = [0]*N
//       for j in range(s, min(s+64, Q)): dp[A[j]-1] |= 1 << (j - s)
//       for x, y in edges_sorted_by_y: dp[y] |= dp[x]
//       for j in range(s, min(s+64, Q)): print('Yes' if dp[B[j]-1] >> (j-s) & 1 else 'No')
// ==========================================================================

use proconio::input;
use std::io::{self, Write};
use std::cmp;

fn main() {
    // Fast I/O
    let stdout = io::stdout();
    // BufWriter で出力をバッファリング。println! を 1e5 回呼ぶより桁違いに速い。
    // Py の sys.stdout.write('\n'.join(...)) に相当。
    let mut out = io::BufWriter::new(stdout.lock());

    // Input reading
    input! {
        n: usize,
        m: usize,
        q: usize,
        edges: [(usize, usize); m],
        queries: [(usize, usize); q],
    }

    // Adjust to zero-based indexing and initialize data
    let mut x = vec![0; m];
    let mut y = vec![0; m];
    let mut g: Vec<i64> = Vec::new();

    for i in 0..m {
        let (xi, yi) = edges[i];
        x[i] = xi - 1;
        y[i] = yi - 1;
        // (y, x) を1つの整数 y*n + x にエンコードしてソートキーにしている。
        // タプルの Vec を sort_by_key(|&(x, y)| (y, x)) すればエンコード不要。
        g.push((y[i] as i64) * n as i64 + x[i] as i64);
    }

    // Sort edges based on encoded values in g
    // 終点 y の昇順に並べると、dp[x] は使われる前に確定している（x < y の DAG なので）。
    g.sort_unstable();
    for i in 0..m {
        x[i] = (g[i] % n as i64) as usize;
        y[i] = (g[i] / n as i64) as usize;
    }

    // Process each query in chunks of 64 for bit manipulation
    for i in 0..((q + 63) / 64) {
        // 64クエリぶんのビット集合。dp[v] の j ビット目 = クエリ j の始点から v に到達できるか。
        let mut dp = vec![0u64; n];
        let chunk_start = i * 64;
        let chunk_end = cmp::min((i + 1) * 64, q);

        // Set initial bits based on queries in the current chunk
        for j in chunk_start..chunk_end {
            let (aj, _) = queries[j];
            // 1 << (j - chunk_start) … dp の要素型 u64 に合わせて推論される。i32 だと 32 以上で溢れるので注意。
            dp[aj - 1] |= 1 << (j - chunk_start);
        }

        // Propagate reachability through edges
        for j in 0..m {
            // 辺 x→y に沿って64クエリ分を一度に OR で伝播。これが bitset 高速化の本体。
            dp[y[j]] |= dp[x[j]];
        }

        // Output results for the current chunk of queries
        for j in chunk_start..chunk_end {
            let (_, bj) = queries[j];
            let answer = if (dp[bj - 1] >> (j - chunk_start)) & 1 == 1 {
                "Yes\n"
            } else {
                "No\n"
            };
            // write!(out, ...) は Result を返すので unwrap。
            write!(out, "{}", answer).unwrap();
        }
    }
}
