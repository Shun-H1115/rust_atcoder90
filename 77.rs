use proconio::input;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i32,
    rev: usize, // reverse edge index in g[to]
}

struct Dinic {
    size: usize,
    level: Vec<i32>,
    iter: Vec<usize>,
    g: Vec<Vec<Edge>>,
}
impl Dinic {
    fn new(sz: usize) -> Self {
        Self {
            size: sz,
            level: vec![-1; sz + 1],
            iter: vec![0; sz + 1],
            g: vec![vec![]; sz + 1],
        }
    }
    fn add_edge(&mut self, u: usize, v: usize, c: i32) {
        let ru = self.g[v].len();
        self.g[u].push(Edge { to: v, cap: c, rev: ru });
        let rv = self.g[u].len() - 1;
        self.g[v].push(Edge { to: u, cap: 0, rev: rv });
    }
    fn bfs(&mut self, s: usize) {
        self.level.fill(-1);
        let mut q = VecDeque::new();
        self.level[s] = 0;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for e in &self.g[v] {
                if e.cap > 0 && self.level[e.to] == -1 {
                    self.level[e.to] = self.level[v] + 1;
                    q.push_back(e.to);
                }
            }
        }
    }
    fn dfs(&mut self, v: usize, t: usize, f: i32) -> i32 {
        if v == t {
            return f;
        }
        let gv_len = self.g[v].len();
        while self.iter[v] < gv_len {
            let i = self.iter[v];
            let to = self.g[v][i].to;
            let cap = self.g[v][i].cap;
            if cap > 0 && self.level[v] < self.level[to] {
                let d = self.dfs(to, t, f.min(cap));
                if d > 0 {
                    let rev = self.g[v][i].rev;
                    self.g[v][i].cap -= d;
                    self.g[to][rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        0
    }
    fn max_flow(&mut self, s: usize, t: usize) -> i32 {
        let mut flow = 0;
        loop {
            self.bfs(s);
            if self.level[t] == -1 {
                break;
            }
            self.iter.fill(0);
            loop {
                let f = self.dfs(s, t, 1_000_000_007);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
}

fn main() {
    input! {
        n: usize,
        tstep: i64,
        a_pts: [(i64, i64); n],
        b_pts: [(i64, i64); n],
    }

    // 1-based に積み替え
    let mut ax = vec![0i64; n + 1];
    let mut ay = vec![0i64; n + 1];
    for i in 1..=n {
        ax[i] = a_pts[i - 1].0;
        ay[i] = a_pts[i - 1].1;
    }
    let mut bx = vec![0i64; n + 1];
    let mut by = vec![0i64; n + 1];
    for i in 1..=n {
        bx[i] = b_pts[i - 1].0;
        by[i] = b_pts[i - 1].1;
    }

    // B点 -> index
    let mut mp: HashMap<(i64, i64), usize> = HashMap::with_capacity(n * 2);
    for i in 1..=n {
        mp.insert((bx[i], by[i]), i);
    }

    // 8方向
    let dx: [i64; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
    let dy: [i64; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

    // グラフ
    let src = 2 * n + 1;
    let sink = 2 * n + 2;
    let mut dinic = Dinic::new(2 * n + 5);

    // Nex と辺構築
    let mut nex = vec![vec![0usize; 8]; n + 1];
    for i in 1..=n {
        for k in 0..8 {
            let tx = ax[i] + dx[k] * tstep;
            let ty = ay[i] + dy[k] * tstep;
            if let Some(&bidx) = mp.get(&(tx, ty)) {
                nex[i][k] = bidx;
                dinic.add_edge(i, n + bidx, 1);
            }
        }
    }
    for i in 1..=n {
        dinic.add_edge(src, i, 1);
        dinic.add_edge(n + i, sink, 1);
    }

    // 最大流
    let res = dinic.max_flow(src, sink);
    if res as usize != n {
        println!("No");
        return;
    }

    // 使用された辺（cap==0 の正向辺）から方向を復元
    let mut answer = vec![0usize; n + 1];
    for i in 1..=n {
        for e in &dinic.g[i] {
            if e.to > 2 * n || e.cap == 1 {
                continue;
            }
            let bidx = e.to - n;
            for k in 0..8 {
                if nex[i][k] == bidx {
                    answer[i] = k + 1; // 1-based direction
                    break;
                }
            }
        }
    }

    println!("Yes");
    for i in 1..=n {
        if i >= 2 {
            print!(" ");
        }
        print!("{}", answer[i]);
    }
    println!();
}