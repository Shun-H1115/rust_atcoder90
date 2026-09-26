// ==========================================================================
// 典型90 #040  Get More Money  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_an
// ==========================================================================
// 【アルゴリズム】 燃やす埋める（最小カット）→ 最大流。Σ A - 最小カット が答え
// 【計算量】       最大流（容量スケーリング Ford-Fulkerson）
// 【学習計画】     第8週以降（★7。フローは2か月の範囲外でOK。通勤で読むだけ）
//
// 【このファイルで覚えるRust文法】
//   - #[derive(Clone)] struct Edge … vec![vec![]; n] で複製するには Clone が必要
//   - 逆辺の添字 rev を持つ隣接リスト（フローの定番データ構造）
//   - 借用の工夫: let (to, cap) = { let e = &g[pos][ei]; (e.to, e.cap) }; で値をコピーしてから借用を終わらせる
//   - &mut [bool] … Vec<bool> を可変スライスとして受け取る
//
// 【Pythonで書くと（考え方の対応）】
//   # ソース→i に容量 A_i、i→シンク に容量 W、依存 c→i に容量 INF
//   # 答え = ΣA - maxflow
//
// 【注意・改善ポイント】
//   ! AtCoder では ac_library::MfGraph（Dinic 法）が使える。自作より速く安全なので、実戦ではこちらを推奨。
// ==========================================================================

use proconio::input;

const INF: i64 = 1_012_345_678;

// Clone を derive しておくと vec![vec![]; n + 2] で Vec<Vec<Edge>> を作れる。
#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i64,
    // rev … g[to] の中で自分の逆辺が何番目か。フローを流したら逆辺の容量を増やす。
    rev: usize, // index of the reverse edge in g[to]
}

// g: &mut Vec<Vec<Edge>> … グラフを書き換えるので可変借用。
fn add_edge(g: &mut Vec<Vec<Edge>>, a: usize, b: usize, cap: i64) {
    let rev_a = g[b].len();
    // Edge { to: b, cap, rev: rev_a } … cap: cap を cap と省略（フィールド初期化の省略記法）。
    g[a].push(Edge { to: b, cap, rev: rev_a });
    let rev_b = g[a].len() - 1;
    g[b].push(Edge { to: a, cap: 0, rev: rev_b });
}

// DFS to find an augmenting path that can push at least `step`.
fn find_augment(pos: usize, tar: usize, step: i64, g: &mut Vec<Vec<Edge>>, vis: &mut [bool]) -> bool {
    if pos == tar {
        return true;
    }
    vis[pos] = true;

    let m = g[pos].len();
    for ei in 0..m {
        // Read edge fields first (avoid borrow issues after recursive call)
        // 【借用チェッカー対策】e = &g[pos][ei] を持ったまま再帰で g を &mut で渡すとコンパイルエラー。
        // ブロック内で必要な値だけコピーし、ブロックを抜けて借用を終わらせてから再帰する。
        let (to, cap) = {
            let e = &g[pos][ei];
            (e.to, e.cap)
        };
        if !vis[to] && cap >= step {
            // g, vis は既に &mut なのでそのまま渡す（再借用）。
            if find_augment(to, tar, step, g, vis) {
                // Augment along this edge
                // 再帰から戻ったあとなら g を書き換えられる。
                g[pos][ei].cap -= step;
                let rev = g[pos][ei].rev;
                g[to][rev].cap += step;
                return true;
            }
        }
    }
    false
}

fn max_flow(src: usize, tar: usize, maxstep: i64, g: &mut Vec<Vec<Edge>>) -> i64 {
    let mut flow: i64 = 0;
    let mut step: i64 = 1;
    // 容量スケーリング: 大きい単位 step で流せるだけ流し、流せなくなったら step を半分に。
    while step * 2 <= maxstep {
        step *= 2;
    }
    loop {
        // 毎回訪問済み配列を作り直す。&mut vis で Vec → &mut [bool] に自動変換される。
        let mut vis = vec![false; g.len()];
        let res = find_augment(src, tar, step, g, &mut vis);
        if !res {
            if step == 1 {
                break;
            }
            step >>= 1;
        } else {
            flow += step;
        }
    }
    flow
}

fn main() {
    input! {
        n: usize,
        w_cap: i64,            // W
    }
    let mut a = vec![0i64; n];
    for i in 0..n {
        input! { ai: i64 }
        a[i] = ai;
    }

    // Graph: nodes 0..=n+1 (0 = source, n+1 = sink)
    let mut g: Vec<Vec<Edge>> = vec![vec![]; n + 2];

    // Read dependencies and add edges with INF capacity: c -> i
    for i in 1..=n {
        input! { k: usize }
        for _ in 0..k {
            input! { c: usize } // note: input uses 1-based vertex indices as in C++
            add_edge(&mut g, c, i, INF);
        }
    }

    // Source -> i with capacity A[i-1], and i -> sink with capacity W
    for i in 1..=n {
        add_edge(&mut g, 0, i, a[i - 1]);
        add_edge(&mut g, i, n + 1, w_cap);
    }

    let res = max_flow(0, n + 1, w_cap, &mut g);
    // a.iter().sum() … sum の型は左辺の : i64 から決まる。
    let sum_a: i64 = a.iter().sum();
    let answer = sum_a - res;
    println!("{}", answer);
}
