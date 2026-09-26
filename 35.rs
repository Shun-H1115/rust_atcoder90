// ==========================================================================
// 典型90 #035  Preserve Connectivity  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ai
// ==========================================================================
// 【アルゴリズム】 LCA（ダブリング）＋オイラーツアー順ソート（DFS順に並べ隣同士の距離を足すと、選んだ頂点を結ぶ部分木の辺数の2倍）
// 【計算量】       O((N + ΣK) log N)
// 【学習計画】     第8週（★7 実装推奨5問の1つ。LCA は頻出なのでライブラリ化必須）
//
// 【このファイルで覚えるRust文法】
//   - input! { mut a: usize, mut b: usize } … 1行ずつ読んで mut で受け取り、1-indexed → 0-indexed に直す
//   - 明示的スタックによる非再帰DFS … Vec を stack として push/pop（Py でも深い再帰回避に使う手法）
//   - ダブリング表 par[k][v] = v の 2^k 個上の祖先
//   - クロージャ lca が depth, par, bits を借用して捕獲
//   - std::mem::swap(&mut a, &mut b) … Py: a, b = b, a
//   - if 式の中で input! を呼んで Vec を返す
//
// 【Pythonで書くと（考え方の対応）】
//   # 1. 根から DFS して行きがけ順 id[v] と depth[v], par[0][v] を求める
//   # 2. par[k][v] = par[k-1][par[k-1][v]] でダブリング表を作る
//   # 3. 各クエリ: 頂点を id 順にソート → 巡回的に隣同士の dist を足して 2 で割る
//   #    dist(u, v) = depth[u] + depth[v] - 2*depth[lca(u, v)]
//
// 【注意・改善ポイント】
//   ! parent_stack は宣言だけで未使用（warning が出る）。削除してよい。
//   ! answer は Σ(depth[vj] - depth[lca]) で計算しており、Σdist/2 と同値になるよう整理されている。
//     （depth[vj] + depth[vk] - 2 depth[p] を巡回和すると depth の項が2回ずつ現れるため）
// ==========================================================================

use proconio::input;

fn main() {
    input! { n: usize }

    // Undirected tree
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        // input! は複数行に分けて何度呼んでもよい。mut で受け取って -1 する。
        input! { mut a: usize, mut b: usize }
        a -= 1;
        b -= 1;
        g[a].push(b);
        g[b].push(a);
    }

    // bits = smallest s.t. (1<<bits) >= n ; ensure at least 1
    let mut bits: usize = 0;
    // 1usize << bits … 型を usize に固定してシフト。Py: (n-1).bit_length()
    while (1usize << bits) < n {
        bits += 1;
    }
    if bits == 0 {
        bits = 1;
    }

    // Binary lifting table and arrays
    // par[k][v]: 頂点 v の 2^k 代上の祖先。根の親は根自身にしておくと範囲外を気にしなくてよい。
    let mut par = vec![vec![0usize; n]; bits]; // par[k][v] = 2^k-th ancestor
    let mut depth = vec![0usize; n];
    let mut id = vec![0usize; n]; // DFS preorder index

    // Iterative DFS to fill par[0], depth, id (preorder from root=0)
    let root = 0usize;
    let mut vert_id: usize = 0;
    // (頂点, 親) のタプルを積むスタック。再帰を使わないので深い木でも安全。
    let mut stack: Vec<(usize, usize)> = vec![(root, root)]; // (v, parent)
    // We'll do manual preorder: when we pop a node first time, we push its children later.
    // To avoid revisits, we only go to neighbors != parent.
    // We also need a separate stack to emulate preorder assignment; easiest is push children after assigning current.
    // （未使用）この変数は使われていない。
    let mut parent_stack: Vec<(usize, usize, usize)> = Vec::new(); // temp to process children
    // while let Some(...) = stack.pop() … スタックが空になるまで。Py: while stack: v, p = stack.pop()
    while let Some((v, p)) = stack.pop() {
        // visit v (preorder)
        par[0][v] = p;
        id[v] = vert_id;
        vert_id += 1;
        if v != p {
            depth[v] = depth[p] + 1;
        }

        // push children
        // g[v].iter().rev() … 逆順に積むと、pop 順が元の隣接順になる（行きがけ順を再帰版と揃えるため）。
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
            // 2^k 上 = 2^(k-1) 上のさらに 2^(k-1) 上。
            par[k][v] = par[k - 1][par[k - 1][v]];
        }
    }

    // LCA function
    // クロージャ引数にも mut を付けられる。depth/par/bits は外側から借用で捕獲。
    let lca = |mut a: usize, mut b: usize| -> usize {
        if depth[a] < depth[b] {
            // std::mem::swap … 2変数の中身を入れ替え。Py: a, b = b, a
            std::mem::swap(&mut a, &mut b);
        }
        // Lift a up to b's depth
        let mut diff = depth[a] - depth[b];
        for k in (0..bits).rev() {
            // diff の各ビットを見て 2^k ずつ持ち上げる（ダブリングの典型）。
            if (diff >> k) & 1 == 1 {
                a = par[k][a];
            }
        }
        if a == b {
            return a;
        }
        // Lift both while their ancestors differ
        // 祖先が一致しない限り両方を持ち上げる → 最後に1つ上が LCA。
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
        // if 式のブロック内で input! して Vec を作り、それを sel の値にしている。
        let mut sel: Vec<usize> = if verts > 0 {
            input! { arr: [usize; verts] }
            // into_iter().map(|x| x - 1).collect() … Py: [x-1 for x in arr]
            arr.into_iter().map(|x| x - 1).collect()
        } else {
            Vec::new()
        };

        // Sort by DFS preorder id
        // オイラーツアー（行きがけ）順にソート。これで隣接ペアの距離和が部分木の辺数×2 になる。
        sel.sort_by_key(|&v| id[v]);

        // Compute answer
        let mut answer: i64 = 0;
        for j in 0..verts {
            let vj = sel[j];
            // (j + 1) % verts … 最後と最初もつなぐ（巡回）。
            let vk = sel[(j + 1) % verts];
            let p = lca(vj, vk);
            // depth は usize なので i64 に変換してから加減算（負の中間値対策）。
            answer += depth[vj] as i64;
            answer -= depth[p] as i64;
        }
        println!("{}", answer);
    }
}
