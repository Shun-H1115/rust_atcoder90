// ==========================================================================
// 典型90 #083  Colorful Graph  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ce
// ==========================================================================
// 【アルゴリズム】 平方分割（次数 √(2M) 以上の「大きい頂点」は遅延更新、小さい頂点は隣接を直接更新）
// 【計算量】       O(Q √M)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - Vec::<usize>::new() … ターボフィッシュで要素型を明示
//   - usize::MAX を「該当なし」の印として使う（Option<usize> でも可）
//   - -1i32 … 「未更新」を -1 で表すため符号付き
//   - for (i, &v) in large.iter().enumerate() … 添字と値を同時に
//
// 【Pythonで書くと（考え方の対応）】
//   B = int((2*M) ** 0.5)
//   large = [v for v in range(N) if len(g[v]) >= B]
//   for i, (x, y) in enumerate(Q):
//       last = max([upd[x]] + [upd_large[j] for j in range(len(large)) if link[x][j]])
//       print(Y[last] if last != -1 else 1)
//       if len(g[x]) < B:
//           upd[x] = i
//           for t in g[x]: upd[t] = i
//       else:
//           upd_large[large_id[x]] = i
//
// 【注意・改善ポイント】
//   ! コメントの「bit-packed in Rust」は誤り。Vec<bool> は1要素1バイトで、ビット詰めはされない。
//     メモリを詰めたいなら u64 のビット列（または bitvec クレート）を使う。
//   ! Q 行の println!。BufWriter で高速化できる。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // Graph (0-indexed)
    let mut g = vec![Vec::<usize>::new(); n];
    for _ in 0..m {
        // 1行の2数を mut で受けて -1。
        input! { mut a: usize, mut b: usize }
        a -= 1;
        b -= 1;
        g[a].push(b);
        g[b].push(a);
    }

    input! { q: usize }
    let mut x = vec![0usize; q];
    let mut y = vec![0i64; q];
    for i in 0..q {
        input! { mut xi: usize, yi: i64 }
        xi -= 1;
        x[i] = xi;
        y[i] = yi;
    }

    // B = floor(sqrt(2M))  (same as C++)
    // 閾値 √(2M)。次数がこれ以上の頂点は高々 √(2M) 個しかない（次数の総和は 2M）。
    let b_thresh = ((2 * m) as f64).sqrt() as usize;

    // Identify "large degree" vertices
    let mut large = Vec::<usize>::new();
    for v in 0..n {
        if g[v].len() >= b_thresh {
            large.push(v);
        }
    }
    let k = large.len();

    // Map vertex -> index in large (or None)
    // usize::MAX を「大きい頂点ではない」の印に。
    let mut large_id = vec![usize::MAX; n];
    for (i, &v) in large.iter().enumerate() {
        large_id[v] = i;
    }

    // link[v][j] = true if v is adjacent to large[j] OR v == large[j]
    // Using Vec<Vec<bool>> just like the C++ (bit-packed in Rust, memory-efficient).
    // link[v][j]: v が大きい頂点 j に隣接している（または自身）か。
    let mut link = vec![vec![false; k]; n];
    for (j, &lv) in large.iter().enumerate() {
        for &to in &g[lv] {
            link[to][j] = true;
        }
        link[lv][j] = true;
    }

    // update[v] = last query index that updated v via a "small" center update
    // update_large[j] = last query index that updated large[j] (lazy)
    // 各頂点が最後に色を塗られたクエリ番号。-1 は初期色。
    let mut update = vec![-1i32; n];
    let mut update_large = vec![-1i32; k];

    for i in 0..q {
        // Find latest update affecting x[i]
        // 直接更新された値と、隣接する大きい頂点の遅延更新の新しい方を取る。
        let mut last = update[x[i]];
        for j in 0..k {
            if link[x[i]][j] {
                last = last.max(update_large[j]);
            }
        }

        // Output current color
        if last != -1 {
            // last は i32 なので as usize で添字に。
            println!("{}", y[last as usize]);
        } else {
            println!("1");
        }

        // Apply update
        // 小さい頂点: 隣接頂点を全部直接書き換え（O(√M)）。
        if g[x[i]].len() < b_thresh {
            // Eagerly update x and its neighbors
            update[x[i]] = i as i32;
            for &to in &g[x[i]] {
                update[to] = i as i32;
            }
        } else {
            // Lazy update for large vertex
            let ptr = large_id[x[i]];
            // ptr must exist because degree is large
            // 大きい頂点: 自分の遅延値だけ更新（O(1)）。読み出し側が link で拾う。
            update_large[ptr] = i as i32;
        }
    }
}
