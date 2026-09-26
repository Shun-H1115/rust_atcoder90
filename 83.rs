use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // Graph (0-indexed)
    let mut g = vec![Vec::<usize>::new(); n];
    for _ in 0..m {
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
    let mut large_id = vec![usize::MAX; n];
    for (i, &v) in large.iter().enumerate() {
        large_id[v] = i;
    }

    // link[v][j] = true if v is adjacent to large[j] OR v == large[j]
    // Using Vec<Vec<bool>> just like the C++ (bit-packed in Rust, memory-efficient).
    let mut link = vec![vec![false; k]; n];
    for (j, &lv) in large.iter().enumerate() {
        for &to in &g[lv] {
            link[to][j] = true;
        }
        link[lv][j] = true;
    }

    // update[v] = last query index that updated v via a "small" center update
    // update_large[j] = last query index that updated large[j] (lazy)
    let mut update = vec![-1i32; n];
    let mut update_large = vec![-1i32; k];

    for i in 0..q {
        // Find latest update affecting x[i]
        let mut last = update[x[i]];
        for j in 0..k {
            if link[x[i]][j] {
                last = last.max(update_large[j]);
            }
        }

        // Output current color
        if last != -1 {
            println!("{}", y[last as usize]);
        } else {
            println!("1");
        }

        // Apply update
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
            update_large[ptr] = i as i32;
        }
    }
}
