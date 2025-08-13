use proconio::input;

fn main() {
    input! { n: usize }

    // Undirected tree
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! { mut a: usize, mut b: usize }
        a -= 1;
        b -= 1;
        g[a].push(b);
        g[b].push(a);
    }

    // bits = smallest s.t. (1<<bits) >= n ; ensure at least 1
    let mut bits: usize = 0;
    while (1usize << bits) < n {
        bits += 1;
    }
    if bits == 0 {
        bits = 1;
    }

    // Binary lifting table and arrays
    let mut par = vec![vec![0usize; n]; bits]; // par[k][v] = 2^k-th ancestor
    let mut depth = vec![0usize; n];
    let mut id = vec![0usize; n]; // DFS preorder index

    // Iterative DFS to fill par[0], depth, id (preorder from root=0)
    let root = 0usize;
    let mut vert_id: usize = 0;
    let mut stack: Vec<(usize, usize)> = vec![(root, root)]; // (v, parent)
    // We'll do manual preorder: when we pop a node first time, we push its children later.
    // To avoid revisits, we only go to neighbors != parent.
    // We also need a separate stack to emulate preorder assignment; easiest is push children after assigning current.
    let mut parent_stack: Vec<(usize, usize, usize)> = Vec::new(); // temp to process children
    while let Some((v, p)) = stack.pop() {
        // visit v (preorder)
        par[0][v] = p;
        id[v] = vert_id;
        vert_id += 1;
        if v != p {
            depth[v] = depth[p] + 1;
        }

        // push children
        for &to in g[v].iter().rev() {
            if to == p {
                continue;
            }
            stack.push((to, v));
        }
    }

    // Build lifting table
    for k in 1..bits {
        for v in 0..n {
            par[k][v] = par[k - 1][par[k - 1][v]];
        }
    }

    // LCA function
    let lca = |mut a: usize, mut b: usize| -> usize {
        if depth[a] < depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        // Lift a up to b's depth
        let mut diff = depth[a] - depth[b];
        for k in (0..bits).rev() {
            if (diff >> k) & 1 == 1 {
                a = par[k][a];
            }
        }
        if a == b {
            return a;
        }
        // Lift both while their ancestors differ
        for k in (0..bits).rev() {
            if par[k][a] != par[k][b] {
                a = par[k][a];
                b = par[k][b];
            }
        }
        par[0][a]
    };

    input! { q: usize }
    for _ in 0..q {
        input! { verts: usize }
        let mut sel: Vec<usize> = if verts > 0 {
            input! { arr: [usize; verts] }
            arr.into_iter().map(|x| x - 1).collect()
        } else {
            Vec::new()
        };

        // Sort by DFS preorder id
        sel.sort_by_key(|&v| id[v]);

        // Compute answer
        let mut answer: i64 = 0;
        for j in 0..verts {
            let vj = sel[j];
            let vk = sel[(j + 1) % verts];
            let p = lca(vj, vk);
            answer += depth[vj] as i64;
            answer -= depth[p] as i64;
        }
        println!("{}", answer);
    }
}