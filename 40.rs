use proconio::input;

const INF: i64 = 1_012_345_678;

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i64,
    rev: usize, // index of the reverse edge in g[to]
}

fn add_edge(g: &mut Vec<Vec<Edge>>, a: usize, b: usize, cap: i64) {
    let rev_a = g[b].len();
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
        let (to, cap) = {
            let e = &g[pos][ei];
            (e.to, e.cap)
        };
        if !vis[to] && cap >= step {
            if find_augment(to, tar, step, g, vis) {
                // Augment along this edge
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
    while step * 2 <= maxstep {
        step *= 2;
    }
    loop {
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
    let sum_a: i64 = a.iter().sum();
    let answer = sum_a - res;
    println!("{}", answer);
}