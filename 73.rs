use proconio::{input, marker::Chars};

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        cs: Chars,            // 長さ N の 'a'/'b' 列を 1 トークンで読み取る想定
    }

    let mut c = vec![' '; n + 1];
    for i in 1..=n {
        c[i] = cs[i - 1];
    }

    let mut g = vec![Vec::<usize>::new(); n + 1];
    for _ in 0..(n - 1) {
        input! { a: usize, b: usize }
        g[a].push(b);
        g[b].push(a);
    }

    // dp[v] = [dp0, dp1, dp2]
    // C[v]=='a' のとき dp0, 'b' のとき dp1 を val1 にして dp2 = val2 - val1
    let mut dp = vec![[0_i64; 3]; n + 1];

    // 反復DFS（後行順）: (v, parent, visited_flag)
    let mut stack: Vec<(usize, usize, bool)> = vec![(1, 0, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            // 先に自分を「後処理」で戻す
            stack.push((v, p, true));
            // 子を積む
            for &to in &g[v] {
                if to == p {
                    continue;
                }
                stack.push((to, v, false));
            }
        } else {
            // 後処理：子の dp が確定している
            let mut val1: i64 = 1;
            let mut val2: i64 = 1;

            for &to in &g[v] {
                if to == p {
                    continue;
                }
                match c[v] {
                    'a' => {
                        // val1 *= (dp[to][0] + dp[to][2]);
                        let t1 = (dp[to][0] + dp[to][2]) % MOD;
                        val1 = (val1 * t1) % MOD;
                        // val2 *= (dp[to][0] + dp[to][1] + 2*dp[to][2]);
                        let t2 = (dp[to][0] + dp[to][1] + (2 * dp[to][2]) % MOD) % MOD;
                        val2 = (val2 * t2) % MOD;
                    }
                    'b' => {
                        // val1 *= (dp[to][1] + dp[to][2]);
                        let t1 = (dp[to][1] + dp[to][2]) % MOD;
                        val1 = (val1 * t1) % MOD;
                        // val2 *= (dp[to][0] + dp[to][1] + 2*dp[to][2]);
                        let t2 = (dp[to][0] + dp[to][1] + (2 * dp[to][2]) % MOD) % MOD;
                        val2 = (val2 * t2) % MOD;
                    }
                    _ => unreachable!("c[v] must be 'a' or 'b'"),
                }
            }
use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    // ---- Input ----
    input! {
        n: usize,
        c_in: [char; n],
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

    // ---- Iterative DFS to get parent + order (avoid recursion limit) ----
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

    // ---- DP ----
    // dp[v][0]: number of ways in subtree(v) such that the component containing v has only 'a'
    // dp[v][1]: number of ways in subtree(v) such that the component containing v has only 'b'
    // dp[v][2]: number of ways in subtree(v) such that the component containing v has both 'a' and 'b'
    //
    // We count edge deletions only inside the subtree (edges to children), and do NOT decide about the edge to parent here.
    let mut dp = vec![[0i64; 3]; n + 1];

    for &v in order.iter().rev() {
        let mut val1: i64 = 1;
        let mut val2: i64 = 1;

        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }

            let d0 = dp[to][0];
            let d1 = dp[to][1];
            let d2 = dp[to][2];

            // For each child edge (v-to), we either keep it (merge components) or cut it.
            // The formulas below compactly represent the same transitions as the original C++.
            match c[v] {
                'a' => {
                    // To keep v's component as "only a":
                    // child side must not introduce 'b' into v's component => allowed: (only a) or (both) but merged in a way?
                    // This matches the original formula: (dp[to][0] + dp[to][2])
                    let t1 = (d0 + d2) % MOD;

                    // For "not forcing v's component to be only a" (i.e., total ways considering cut/keep),
                    // original uses: (dp[to][0] + dp[to][1] + 2*dp[to][2])
                    let t2 = (d0 + d1 + 2 * d2) % MOD;

                    val1 = (val1 * t1) % MOD;
                    val2 = (val2 * t2) % MOD;
                }
                'b' => {
                    let t1 = (d1 + d2) % MOD;
                    let t2 = (d0 + d1 + 2 * d2) % MOD;

                    val1 = (val1 * t1) % MOD;
                    val2 = (val2 * t2) % MOD;
                }
                _ => unreachable!("c[i] must be 'a' or 'b'"),
            }
        }

        match c[v] {
            'a' => {
                dp[v][0] = val1;
                dp[v][2] = (val2 - val1 + MOD) % MOD;
            }
            'b' => {
                dp[v][1] = val1;
                dp[v][2] = (val2 - val1 + MOD) % MOD;
            }
            _ => unreachable!(),
        }
    }

    // We need all connected components to contain both letters.
    // That means the (only) component containing the root after all deletions must be "both",
    // and since every component is somewhere in the tree, dp[root][2] exactly counts valid deletions.
    println!("{}", dp[root][2] % MOD);
}

            match c[v] {
                'a' => {
                    dp[v][0] = val1 % MOD;
                    dp[v][2] = (val2 - val1).rem_euclid(MOD);
                }
                'b' => {
                    dp[v][1] = val1 % MOD;
                    dp[v][2] = (val2 - val1).rem_euclid(MOD);
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{}", dp[1][2] % MOD);
}