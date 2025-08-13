use proconio::{input, marker::Chars};
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

struct Precomp {
    h: usize,
    w: usize,
    // "used" is only touched within a 2-row sliding window in DFS,
    // but we allocate h+2 rows for safety.
    used: Vec<Vec<bool>>,
    // Per-column state count
    cnt: Vec<usize>,
    // Per-column list of state strings (each of length W+1)
    state: Vec<Vec<String>>,
    // Per-column map: state string -> (index in `state[col]`, can_place_next)
    maps: Vec<HashMap<String, (usize, bool)>>,
}

impl Precomp {
    fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            used: vec![vec![false; w]; h + 2],
            cnt: vec![0; w],
            state: (0..w).map(|_| Vec::new()).collect(),
            maps: (0..w).map(|_| HashMap::new()).collect(),
        }
    }

    // Check if (sx, sy) can be used (no selected cell in its 8-neighborhood).
    // Note: the original C++ only checked ty bounds and tx >= 0, not tx >= H.
    // We mirror that but guard indices safely in Rust.
    fn hantei(&self, sx: isize, sy: isize) -> bool {
        const DX: [isize; 8] = [1, 1, 1, 0, -1, -1, -1, 0];
        const DY: [isize; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
        for dir in 0..8 {
            let tx = sx + DX[dir];
            let ty = sy + DY[dir];
            if tx < 0 || ty < 0 || ty >= self.w as isize {
                continue;
            }
            // Safe guard for row bound (C++ didn't check tx >= H explicitly)
            if tx as usize >= self.used.len() {
                continue;
            }
            if self.used[tx as usize][ty as usize] {
                return false;
            }
        }
        true
    }

    // DFS over a window of length W+1 starting at absolute position `pos`.
    // `dep` is current depth (0..=W+1), `s` is the bit string built so far.
    fn dfs(&mut self, pos: usize, dep: usize, mut s: String) {
        let sx = (pos / self.w) as isize;
        let sy = (pos % self.w) as isize;

        if dep == self.w + 1 {
            // Window finished: record state into the column corresponding to the *next* cell
            // (which is (pos % W) = (start + W + 1) % W).
            let col = sy as usize;
            let idx = self.cnt[col];
            let flag = self.hantei(sx, sy); // whether we can place at the next cell
            self.state[col].push(s.clone());
            self.maps[col].insert(s, (idx, flag));
            self.cnt[col] += 1;
            return;
        }

        // Choice 0: place '0'
        {
            let mut s0 = s.clone();
            s0.push('0');
            self.dfs(pos + 1, dep + 1, s0);
        }

        // Choice 1: place '1' if allowed
        if self.hantei(sx, sy) {
            // mark used
            self.used[sx as usize][sy as usize] = true;
            s.push('1');
            self.dfs(pos + 1, dep + 1, s);
            // unmark
            self.used[sx as usize][sy as usize] = false;
        }
    }

    // Run DFS starting at each column `i` over a window length W+1.
    fn build_states(&mut self) {
        for i in 0..self.w {
            self.dfs(i, 0, String::new());
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let mut pre = Precomp::new(h, w);
    pre.build_states();

    // Build transition tables nex0 / nex1 for each column and each state index.
    // nex0: shift-left + '0' (always allowed)
    // nex1: shift-left + '1' (only if current state's `flag` is true)
    let mut nex0: Vec<Vec<i32>> = vec![Vec::new(); w];
    let mut nex1: Vec<Vec<i32>> = vec![Vec::new(); w];

    for i in 0..w {
        let cnt_i = pre.cnt[i];
        nex0[i] = vec![-1; cnt_i];
        nex1[i] = vec![-1; cnt_i];

        for j in 0..cnt_i {
            let t = &pre.state[i][j];
            // shift and append
            let tail = if t.len() >= 1 { &t[1..] } else { "" };
            let t0 = {
                let mut s = String::with_capacity(t.len());
                s.push_str(tail);
                s.push('0');
                s
            };
            let t1 = {
                let mut s = String::with_capacity(t.len());
                s.push_str(tail);
                s.push('1');
                s
            };

            // Next column is (i+1) % W
            let ni = (i + 1) % w;

            // nex0 always exists
            let to0 = pre.maps[ni]
                .get(&t0)
                .expect("t0 must exist in next column map")
                .0 as i32;
            nex0[i][j] = to0;

            // nex1 only if current state's flag is true
            let can_place = pre.maps[i]
                .get(t)
                .expect("current state must exist")
                .1;
            if can_place {
                let to1 = pre.maps[ni]
                    .get(&t1)
                    .expect("t1 must exist in next column map")
                    .0 as i32;
                nex1[i][j] = to1;
            } else {
                nex1[i][j] = -1;
            }
        }
    }

    // DP: dp[i][j][k]
    // i: row index (0..=h), j: col index (0..w-1), k: state index in column j (0..cnt[j]-1)
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![Vec::new(); w]; h + 1];
    for i in 0..=h {
        for j in 0..w {
            dp[i][j] = vec![0_i64; pre.cnt[j]];
        }
    }
    dp[0][0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            let (mut n1, mut n2) = (i, j + 1);
            if n2 == w {
                n1 += 1;
                n2 = 0;
            }
            for k in 0..pre.cnt[j] {
                let val = dp[i][j][k];
                if val == 0 {
                    continue;
                }
                // choose 0
                let to0 = nex0[j][k] as usize;
                dp[n1][n2][to0] += val;
                if dp[n1][n2][to0] >= MOD {
                    dp[n1][n2][to0] -= MOD;
                }

                // choose 1 (only if allowed by nex1 and the cell is '.')
                if nex1[j][k] != -1 && grid[i][j] == '.' {
                    let to1 = nex1[j][k] as usize;
                    dp[n1][n2][to1] += val;
                    if dp[n1][n2][to1] >= MOD {
                        dp[n1][n2][to1] -= MOD;
                    }
                }
            }
        }
    }

    // Answer: sum over dp[h][0][*]
    let mut ans = 0_i64;
    for v in &dp[h][0] {
        ans += *v;
        if ans >= MOD {
            ans -= MOD;
        }
    }
    println!("{}", ans % MOD);
}