use proconio::input;
use std::io::{self, Write};
use std::cmp;

fn main() {
    // Fast I/O
    let stdout = io::stdout();
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
        g.push((y[i] as i64) * n as i64 + x[i] as i64);
    }

    // Sort edges based on encoded values in g
    g.sort_unstable();
    for i in 0..m {
        x[i] = (g[i] % n as i64) as usize;
        y[i] = (g[i] / n as i64) as usize;
    }

    // Process each query in chunks of 64 for bit manipulation
    for i in 0..((q + 63) / 64) {
        let mut dp = vec![0u64; n];
        let chunk_start = i * 64;
        let chunk_end = cmp::min((i + 1) * 64, q);

        // Set initial bits based on queries in the current chunk
        for j in chunk_start..chunk_end {
            let (aj, _) = queries[j];
            dp[aj - 1] |= 1 << (j - chunk_start);
        }

        // Propagate reachability through edges
        for j in 0..m {
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
            write!(out, "{}", answer).unwrap();
        }
    }
}
